/*! coi-serviceworker v0.1.7 - Guido Zuidhof, licensed under MIT */
let coepCredentialless = false;
if (typeof window === 'undefined') {
  self.addEventListener("install", () => self.skipWaiting());
  self.addEventListener("activate", (event) => event.waitUntil(self.clients.claim()));
  self.addEventListener("message", (ev) => {
    if (ev.data && ev.data.type === "deregister") {
      self.registration.unregister().then(() => {
        return self.clients.matchAll();
      }).then(clients => {
        clients.forEach(client => client.navigate(client.url));
      });
    }
  });
  self.addEventListener("fetch", function (event) {
    const request = event.request;
    if (request.cache === "only-if-cached" && request.mode !== "same-origin") {
      return;
    }
    event.respondWith(
      fetch(request).then((response) => {
        if (response.status === 0) {
          return response;
        }
        const newHeaders = new Headers(response.headers);
        newHeaders.set("Cross-Origin-Embedder-Policy", coepCredentialless ? "credentialless" : "require-corp");
        newHeaders.set("Cross-Origin-Opener-Policy", "same-origin");
        return new Response(response.body, {
          status: response.status,
          statusText: response.statusText,
          headers: newHeaders,
        });
      }).catch((e) => console.error(e))
    );
  });
} else {
  (() => {
    const reloadedBySelf = window.sessionStorage.getItem("coiReloadedBySelf");
    window.sessionStorage.removeItem("coiReloadedBySelf");
    const coi = {
      state: "unsupported",
      reason: "",
    };
    window.coiServiceWorker = coi;
    const n = navigator;
    if (n.serviceWorker) {
      n.serviceWorker.register(window.document.currentScript ? window.document.currentScript.src : "coi-serviceworker.js").then(
        (registration) => {
          coi.state = "registered";
          registration.addEventListener("updatefound", () => {
            coi.state = "reloading";
            window.location.reload();
          });
          if (registration.active && !n.serviceWorker.controller) {
            coi.state = "reloading";
            window.sessionStorage.setItem("coiReloadedBySelf", "true");
            window.location.reload();
          }
        },
        (err) => {
          coi.state = "error";
          coi.reason = err.message;
        }
      );
    }
  })();
}
