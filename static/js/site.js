window.addEventListener("scroll", (e) => {
    if (window.scrollY > 0) {
        document.querySelector(".header").classList.add("is-visible");
    } else {
        document.querySelector(".header").classList.remove("is-visible");
    }
})