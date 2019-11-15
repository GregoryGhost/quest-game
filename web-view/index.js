import("./pkg").then(module => {
    module.run_app();
    console.log("kek");
    console.log(module.get_number());
});