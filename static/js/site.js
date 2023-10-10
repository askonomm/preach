window.addEventListener("scroll", (e) => {
    if (window.innerWidth > 800) {
        if (window.scrollY > 0) {
            document.querySelector(".header").classList.add("is-visible");
        } else {
            document.querySelector(".header").classList.remove("is-visible");
        }
    }
})

window.addEventListener("load", () => {
    if (window.innerWidth < 800) {
        document.querySelector(".header").classList.add("is-visible");
    }
})

window.addEventListener("resize", () => {
    if (window.innerWidth < 800) {
        document.querySelector(".header").classList.add("is-visible");
    }

    if (window.innerWidth > 800 && window.scrollY === 0) {
        document.querySelector(".header").classList.remove("is-visible");
    }
})