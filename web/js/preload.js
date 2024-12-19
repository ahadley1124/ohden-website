document.addEventListener("DOMContentLoaded", function () {
  function preloadPage(url) {
    // Check if the page is already preloaded
    if (sessionStorage.getItem(url)) {
      // Check if the page was preloaded more than 5 minutes ago
      const time = sessionStorage.getItem(url + "_time");
      if (time && new Date().getTime() - time > 300000) {
        console.log("Preloading page:", url);
        fetch(url)
          .then((response) => response.text())
          .then((data) => {
            const time = new Date().getTime();
            sessionStorage.setItem(url, data);
            sessionStorage.setItem(url + "_time", time);
          })
          .catch((error) => console.error("Error preloading page:", error));
      }
    } else {
      console.log("Preloading page:", url);
      fetch(url)
        .then((response) => response.text())
        .then((data) => {
          const time = new Date().getTime();
          sessionStorage.setItem(url, data);
          sessionStorage.setItem(url + "_time", time);
        })
        .catch((error) => console.error("Error preloading page:", error));
    }
  }

  document.querySelectorAll("a").forEach((link) => {
    link.addEventListener("mouseover", function () {
      preloadPage(this.href);
    });

    link.addEventListener("click", function (event) {
      event.preventDefault();
      const preloadedContent = sessionStorage.getItem(this.href);
      if (preloadedContent) {
        document.body.innerHTML = preloadedContent;
      }
      window.location.href = this.href;
    });
  });
});
