init();

function init() {
    console.log("Running init...");
}


function add_note() {
   const subject = document.getElementById("subjectInput");
    const grade = document.getElementById("gradeInput");

    //add_note(zeitraum_id, fach_id, schriftlich, muendlich, gewichtung)
    eel.add_note(1, 1, 13, 15, 0.6);
}
