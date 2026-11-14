(() => {
  'use strict';

  const STORAGE_KEY = 'clouds.touchControls'; // 'auto' | 'on' | 'off'
  const isTouchDevice = () =>
    window.matchMedia('(pointer: coarse)').matches || (navigator.maxTouchPoints || 0) > 0;

  let mode = localStorage.getItem(STORAGE_KEY) || 'auto';
  let engineRunning = false;
  let sneakLocked = false;

  const isEnabled = () => (mode === 'on' ? true : mode === 'off' ? false : isTouchDevice());

  // 1. Inject mobile-first CSS & touch overlay styles
  const style = document.createElement('style');
  style.textContent = `
    html, body, #clouds-canvas {
      height: 100dvh;
      touch-action: none;
      overscroll-behavior: none;
    }
    #clouds-canvas, .clouds-touch-overlay, .clouds-touch-overlay * {
      user-select: none;
      -webkit-user-select: none;
      -webkit-touch-callout: none;
      touch-action: none;
    }
    @media (max-width: 640px), (pointer: coarse) {
      .modal, #onboarding-modal > *, #options-modal > *, #error-modal > * {
        max-width: 94vw !important;
        max-height: 90dvh !important;
        overflow-y: auto !important;
      }
      .hud-bar {
        padding: env(safe-area-inset-top, 6px) 10px 6px;
        gap: 6px;
      }
    }
    body.touch-controls-active #pointer-status-pill {
      display: none !important;
    }
    .clouds-touch-overlay {
      position: fixed;
      inset: 0;
      z-index: 25;
      pointer-events: none;
      padding: env(safe-area-inset-top, 12px) env(safe-area-inset-right, 14px)
               env(safe-area-inset-bottom, 16px) env(safe-area-inset-left, 14px);
    }
    .clouds-touch-overlay[hidden] { display: none !important; }

    .tc-joystick {
      position: absolute;
      left: max(18px, env(safe-area-inset-left, 18px));
      bottom: max(22px, env(safe-area-inset-bottom, 22px));
      width: 132px;
      height: 132px;
      border-radius: 50%;
      background: rgba(15, 18, 24, 0.42);
      border: 2px solid rgba(255, 255, 255, 0.22);
      backdrop-filter: blur(4px);
      pointer-events: auto;
      display: flex;
      align-items: center;
      justify-content: center;
    }
    .tc-joystick__knob {
      width: 54px;
      height: 54px;
      border-radius: 50%;
      background: rgba(255, 255, 255, 0.78);
      box-shadow: 0 2px 10px rgba(0, 0, 0, 0.35);
      transform: translate(0px, 0px);
      transition: transform 0.05s linear;
      pointer-events: none;
    }
    .tc-actions {
      position: absolute;
      right: max(18px, env(safe-area-inset-right, 18px));
      bottom: max(22px, env(safe-area-inset-bottom, 22px));
      display: grid;
      grid-template-columns: repeat(2, 58px);
      gap: 10px;
      pointer-events: auto;
    }
    .tc-top-actions {
      position: absolute;
      top: 54px;
      right: max(14px, env(safe-area-inset-right, 14px));
      display: flex;
      gap: 8px;
      pointer-events: auto;
    }
    .tc-btn {
      width: 58px;
      height: 58px;
      border-radius: 14px;
      border: 1.5px solid rgba(255, 255, 255, 0.28);
      background: rgba(15, 18, 24, 0.48);
      color: #f4f6fa;
      font: 600 12px/1 system-ui, sans-serif;
      display: flex;
      align-items: center;
      justify-content: center;
      backdrop-filter: blur(4px);
    }
    .tc-btn--sm {
      width: auto;
      height: 36px;
      padding: 0 12px;
      border-radius: 10px;
      font-size: 12px;
    }
    .tc-btn.is-active {
      background: rgba(74, 222, 128, 0.38);
      border-color: rgba(74, 222, 128, 0.85);
    }
  `;
  document.head.appendChild(style);

  // 2. Build touch HUD overlay
  const overlay = document.createElement('div');
  overlay.className = 'clouds-touch-overlay';
  overlay.hidden = true;
  overlay.innerHTML = `
    <div class="tc-top-actions">
      <button type="button" class="tc-btn tc-btn--sm" data-tc-key="KeyE">Bag</button>
      <button type="button" class="tc-btn tc-btn--sm" data-tc-key="Escape">Pause</button>
    </div>
    <div class="tc-joystick" id="tc-joystick" aria-label="Movement joystick">
      <div class="tc-joystick__knob" id="tc-joystick-knob"></div>
    </div>
    <div class="tc-actions">
      <button type="button" class="tc-btn" data-tc-mouse="0" data-tc-look="1">Break</button>
      <button type="button" class="tc-btn" data-tc-mouse="2" data-tc-look="1">Use</button>
      <button type="button" class="tc-btn" data-tc-sneak="1">Sneak</button>
      <button type="button" class="tc-btn" data-tc-key="Space">Jump</button>
    </div>
  `;
  document.body.appendChild(overlay);

  const canvas = document.getElementById('clouds-canvas');
  const joystick = document.getElementById('tc-joystick');
  const knob = document.getElementById('tc-joystick-knob');

  // 3. Synthetic input helpers
  const activeKeys = new Set();
  const keyMap = {
    KeyW: 'w', KeyA: 'a', KeyS: 's', KeyD: 'd',
    Space: ' ', ShiftLeft: 'Shift', ControlLeft: 'Control',
    KeyE: 'e', Escape: 'Escape'
  };

  function setKey(code, down) {
    if (down === activeKeys.has(code)) return;
    if (down) activeKeys.add(code);
    else activeKeys.delete(code);
    const ev = new KeyboardEvent(down ? 'keydown' : 'keyup', {
      code,
      key: keyMap[code] || code,
      bubbles: true,
      cancelable: true
    });
    (canvas || window).dispatchEvent(ev);
  }

  function sendMouse(button, down) {
    if (!canvas) return;
    const rect = canvas.getBoundingClientRect();
    canvas.dispatchEvent(new MouseEvent(down ? 'mousedown' : 'mouseup', {
      button,
      buttons: down ? (button === 0 ? 1 : 2) : 0,
      clientX: rect.left + rect.width / 2,
      clientY: rect.top + rect.height / 2,
      bubbles: true,
      cancelable: true
    }));
  }

  function emitLook(dx, dy) {
    if (window.CloudsWeb && typeof window.CloudsWeb.onTouchLook === 'function') {
      window.CloudsWeb.onTouchLook(dx, dy);
    }
    if (canvas) {
      canvas.dispatchEvent(new MouseEvent('mousemove', {
        movementX: dx,
        movementY: dy,
        bubbles: true
      }));
    }
  }

  // 4. Virtual joystick (multi-touch safe via pointerId)
  let joyPointerId = null;
  const maxRadius = 44;
  const deadzone = 0.22;

  function updateJoystick(clientX, clientY) {
    const rect = joystick.getBoundingClientRect();
    const cx = rect.left + rect.width / 2;
    const cy = rect.top + rect.height / 2;
    let dx = clientX - cx;
    let dy = clientY - cy;
    const dist = Math.hypot(dx, dy);
    if (dist > maxRadius) {
      dx = (dx / dist) * maxRadius;
      dy = (dy / dist) * maxRadius;
    }
    knob.style.transform = `translate(${dx}px, ${dy}px)`;

    const nx = dx / maxRadius;
    const ny = dy / maxRadius;
    setKey('KeyW', ny < -deadzone);
    setKey('KeyS', ny > deadzone);
    setKey('KeyA', nx < -deadzone);
    setKey('KeyD', nx > deadzone);
    setKey('ControlLeft', ny < -0.85); // auto-sprint when pushed forward
  }

  function resetJoystick() {
    joyPointerId = null;
    knob.style.transform = 'translate(0px, 0px)';
    ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'ControlLeft'].forEach((k) => setKey(k, false));
  }

  joystick.addEventListener('pointerdown', (e) => {
    if (joyPointerId !== null) return;
    joyPointerId = e.pointerId;
    joystick.setPointerCapture(e.pointerId);
    updateJoystick(e.clientX, e.clientY);
  });
  joystick.addEventListener('pointermove', (e) => {
    if (e.pointerId === joyPointerId) updateJoystick(e.clientX, e.clientY);
  });
  joystick.addEventListener('pointerup', (e) => {
    if (e.pointerId === joyPointerId) resetJoystick();
  });
  joystick.addEventListener('pointercancel', (e) => {
    if (e.pointerId === joyPointerId) resetJoystick();
  });

  // 5. Action buttons (including moveView drag-to-aim on Break / Use)
  overlay.querySelectorAll('.tc-btn').forEach((btn) => {
    let btnPointerId = null;
    let lastX = 0;
    let lastY = 0;

    btn.addEventListener('pointerdown', (e) => {
      e.preventDefault();
      btnPointerId = e.pointerId;
      lastX = e.clientX;
      lastY = e.clientY;
      btn.setPointerCapture(e.pointerId);

      if (btn.dataset.tcSneak) {
        sneakLocked = !sneakLocked;
        btn.classList.toggle('is-active', sneakLocked);
        setKey('ShiftLeft', sneakLocked);
        return;
      }
      btn.classList.add('is-active');
      if (btn.dataset.tcKey) setKey(btn.dataset.tcKey, true);
      if (btn.dataset.tcMouse !== undefined) sendMouse(Number(btn.dataset.tcMouse), true);
    });

    btn.addEventListener('pointermove', (e) => {
      if (e.pointerId !== btnPointerId || !btn.dataset.tcLook) return;
      emitLook((e.clientX - lastX) * 1.4, (e.clientY - lastY) * 1.4);
      lastX = e.clientX;
      lastY = e.clientY;
    });

    const release = (e) => {
      if (e.pointerId !== btnPointerId) return;
      btnPointerId = null;
      if (!btn.dataset.tcSneak) btn.classList.remove('is-active');
      if (btn.dataset.tcKey) setKey(btn.dataset.tcKey, false);
      if (btn.dataset.tcMouse !== undefined) sendMouse(Number(btn.dataset.tcMouse), false);
    };
    btn.addEventListener('pointerup', release);
    btn.addEventListener('pointercancel', release);
  });

  // 6. Viewport look + tap-to-use / hold-to-break on #clouds-canvas
  let viewPointerId = null;
  let viewStartX = 0;
  let viewStartY = 0;
  let viewLastX = 0;
  let viewLastY = 0;
  let viewMoved = false;
  let viewHoldingBreak = false;
  let holdTimer = null;

  if (canvas) {
    // Prevent pointer lock on touch when touch controls are enabled
    canvas.addEventListener('mousedown', (e) => {
      if (isEnabled() && e.isTrusted) e.stopImmediatePropagation();
    }, true);

    canvas.addEventListener('pointerdown', (e) => {
      if (!isEnabled() || !engineRunning || e.pointerType === 'mouse') return;
      if (viewPointerId !== null) return;
      viewPointerId = e.pointerId;
      viewStartX = viewLastX = e.clientX;
      viewStartY = viewLastY = e.clientY;
      viewMoved = false;
      viewHoldingBreak = false;
      canvas.setPointerCapture(e.pointerId);

      holdTimer = setTimeout(() => {
        if (!viewMoved && viewPointerId === e.pointerId) {
          viewHoldingBreak = true;
          sendMouse(0, true);
        }
      }, 260);
    });

    canvas.addEventListener('pointermove', (e) => {
      if (e.pointerId !== viewPointerId) return;
      const dx = e.clientX - viewLastX;
      const dy = e.clientY - viewLastY;
      if (!viewMoved && Math.hypot(e.clientX - viewStartX, e.clientY - viewStartY) > 10) {
        viewMoved = true;
        clearTimeout(holdTimer);
      }
      if (viewMoved) emitLook(dx * 1.5, dy * 1.5);
      viewLastX = e.clientX;
      viewLastY = e.clientY;
    });

    const endView = (e) => {
      if (e.pointerId !== viewPointerId) return;
      clearTimeout(holdTimer);
      if (viewHoldingBreak) {
        sendMouse(0, false);
      } else if (!viewMoved) {
        sendMouse(2, true);
        setTimeout(() => sendMouse(2, false), 50);
      }
      viewPointerId = null;
    };
    canvas.addEventListener('pointerup', endView);
    canvas.addEventListener('pointercancel', endView);
  }

  // 7. Sync visibility with modals + inject Options toggle
  function syncVisibility() {
    const enabled = isEnabled();
    document.body.classList.toggle('touch-controls-active', enabled);
    const modalOpen = ['onboarding-modal', 'options-modal', 'error-modal'].some((id) => {
      const el = document.getElementById(id);
      return el && !el.hidden;
    });
    overlay.hidden = !(enabled && engineRunning && !modalOpen);
  }

  function injectOptionsToggle() {
    const panel = document.getElementById('panel-general');
    if (!panel || document.getElementById('select-touch-controls')) return;
    const row = document.createElement('div');
    row.className = 'toggle-row';
    row.style.marginTop = '12px';
    row.innerHTML = `
      <div>
        <div style="font-weight:600">Touch Controls</div>
        <div style="font-size:12px;opacity:0.75">On-screen joystick and buttons for phones</div>
      </div>
      <select id="select-touch-controls" style="padding:6px 10px;border-radius:8px;">
        <option value="auto">Auto (Phone)</option>
        <option value="on">Always On</option>
        <option value="off">Off</option>
      </select>
    `;
    panel.appendChild(row);
    const select = row.querySelector('#select-touch-controls');
    select.value = mode;
    select.addEventListener('change', () => {
      mode = select.value;
      localStorage.setItem(STORAGE_KEY, mode);
      syncVisibility();
    });
  }

  window.addEventListener('clouds:launch', () => {
    engineRunning = true;
    syncVisibility();
  });

  ['onboarding-modal', 'options-modal', 'error-modal'].forEach((id) => {
    const el = document.getElementById(id);
    if (el) new MutationObserver(syncVisibility).observe(el, { attributes: true, attributeFilter: ['hidden'] });
  });

  injectOptionsToggle();
  syncVisibility();

  window.CloudsTouch = {
    isEnabled,
    getMode: () => mode,
    setMode: (next) => {
      mode = next;
      localStorage.setItem(STORAGE_KEY, mode);
      const sel = document.getElementById('select-touch-controls');
      if (sel) sel.value = mode;
      syncVisibility();
    }
  };
})();
