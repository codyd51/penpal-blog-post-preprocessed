import snapshot_0_init, { game_start as snapshot_0_entry } from "/writing-about-writing-about-programming/wasm-programs/snapshot_0/pkg/snapshot_0.js";
import snapshot_1_init, { game_start as snapshot_1_entry } from "/writing-about-writing-about-programming/wasm-programs/snapshot_1/pkg/snapshot_1.js";
import snapshot_2_init, { game_start as snapshot_2_entry } from "/writing-about-writing-about-programming/wasm-programs/snapshot_2/pkg/snapshot_2.js";
import snapshot_3_init, { game_start as snapshot_3_entry } from "/writing-about-writing-about-programming/wasm-programs/snapshot_3/pkg/snapshot_3.js";
import snapshot_4_init, { game_start as snapshot_4_entry } from "/writing-about-writing-about-programming/wasm-programs/snapshot_4/pkg/snapshot_4.js";
import snapshot_5_init, { game_start as snapshot_5_entry, handle_input as snapshot_5_handle_input } from "/writing-about-writing-about-programming/wasm-programs/snapshot_5/pkg/snapshot_5.js";
import snapshot_6_init, { game_start as snapshot_6_entry, handle_input as snapshot_6_handle_input } from "/writing-about-writing-about-programming/wasm-programs/snapshot_6/pkg/snapshot_6.js";
import snapshot_7_init, { game_start as snapshot_7_entry, handle_input as snapshot_7_handle_input } from "/writing-about-writing-about-programming/wasm-programs/snapshot_7/pkg/snapshot_7.js";
import snapshot_8_init, { game_start as snapshot_8_entry, handle_input as snapshot_8_handle_input } from "/writing-about-writing-about-programming/wasm-programs/snapshot_8/pkg/snapshot_8.js";

const program_name_to_dispatch = {
    "snapshot_0": [snapshot_0_init, snapshot_0_entry, null],
    "snapshot_1": [snapshot_1_init, snapshot_1_entry, null],
    "snapshot_2": [snapshot_2_init, snapshot_2_entry, null],
    "snapshot_3": [snapshot_3_init, snapshot_3_entry, null],
    "snapshot_4": [snapshot_4_init, snapshot_4_entry, null],
    "snapshot_5": [snapshot_5_init, snapshot_5_entry, snapshot_5_handle_input],
    "snapshot_6": [snapshot_6_init, snapshot_6_entry, snapshot_6_handle_input],
    "snapshot_7": [snapshot_7_init, snapshot_7_entry, snapshot_7_handle_input],
    "snapshot_8": [snapshot_8_init, snapshot_8_entry, snapshot_8_handle_input],
};

$(document).ready(function(){
    const executable_containers = $('.executable_container');
    let window_width = $(window).width();
    if (window_width < 500) {
        executable_containers.each(function() {
            console.log(`Adjusting width... `);
            $(this).css("font-size", 3.2);
        });
        // Slightly adjust the CSS of the power button

        $('.penpal_program_container').each(function() {
            $(this).css("top", "66%");
        });
    }

    let touch_start_x = 0;
    let touch_end_x = 0;
    let touch_start_y = 0;
    let touch_end_y = 0;

    function checkDirection() {
        let direction = null;
        if (touch_end_x < touch_start_x) {
            direction = "Left";
        }
        else if (touch_end_x > touch_start_x) {
            direction = "Right";
        }
        else if (touch_end_y < touch_start_y) {
            direction = "Up";
        }
        else if (touch_end_y > touch_start_y) {
            direction = "Down";
        }
        return direction;
    }

    document.addEventListener('touchstart', e => {
        touch_start_x = e.changedTouches[0].screenX
        touch_start_y = e.changedTouches[0].screenY
    });

    document.addEventListener('touchend', e => {
        touch_end_x = e.changedTouches[0].screenX
        touch_end_y = e.changedTouches[0].screenY
        let direction = checkDirection();
        if (direction === null) {
            return;
        }
        for (const [snapshot_name, callbacks] of Object.entries(program_name_to_dispatch)) {
            // TODO(PT): Does anything bad happen if we call init twice? What if we don't call it?
            const computer_container_selector = `#computer_container_for_${snapshot_name}`;
            const program_container_selector = `#output_for_${snapshot_name}`;
            // Ref: https://stackoverflow.com/questions/8981463/detect-if-hovering-over-element-with-jquery
            if ($(computer_container_selector + ":hover").length === 0) {
                //console.log(`Mouse is not hovered over ${snapshot_name}`);
                continue;
            }
            // The mouse is hovered over this element
            console.log(`Mouse is hovered over ${snapshot_name}!`);
            // Inhibit the default action (scroll / move caret)
            e.preventDefault();

            const [init, entry, maybe_handle_input] = callbacks;
            if (!maybe_handle_input) {
                // Don't try to handle the arrow key here as we don't have an input handler for this program
                continue;
            }

            const program_container = $(program_container_selector);
            const input_repr = `<swipe ${direction}>`;
            const new_text = maybe_handle_input(direction);
            program_container.text(`${program_container.text()}\n${input_repr}\n${new_text}`);

            program_container.scrollTop(program_container.prop("scrollHeight"));
        }
    });

    // TODO(PT): Rename to power_buttons?
    const programContainers = $('.penpal_program_container');
    programContainers.each(function() {
        console.log($(this));

        const classes = $(this).attr("class");
        const programNameClass = classes.split(/\s+/).find((val, i) => {
            return val.startsWith("program_");
        });
        const programName = programNameClass.replace("program_", "");
        const outputDiv = $(`#output_for_${programName}`);

        console.log(`programNameToDis ${program_name_to_dispatch}`);
        console.log(`programName ${programName}`);
        const [init, entry, maybe_handle_input] = program_name_to_dispatch[programName];

        $(this).click(function() {
            $(this).css("animation", "none");
            $(this).css("-webkit-filter", "drop-shadow(0px 0px 4px rgba(255, 255, 255, 1.0)");
            init().then(() => {
                console.log(`Calling into rust...`);
                let output = entry();
                //console.log(`Got output ${output}`);
                //console.log(`outputDiv ${outputDiv}`);
                //preformatted_container.text(output);
                outputDiv.text(output);
                outputDiv.scrollTop(outputDiv.prop("scrollHeight"));
            });
        });
    });

    // Fix the width of each background
    const titleBackgrounds = $('.computer_title_background');
    titleBackgrounds.each(function() {
        $(this).css("width", `${$(this).width() * 1.2}px`);
    })

    const titles = $('.computer_title');
    titles.each(function() {
        $(this).attr("fullText", $(this).text());
        console.log(`${$(this).attr("fullText")}`)
        $(this).data("needsAnimate", true);
        $(this).data("isAnimationInProgress", false);
        $(this).text("$ ");
    })

    function addAnotherCharacter(event) {
        let elem = event.data.elem;
        let i = event.data.i;
        let txt = elem.attr("fullText");

        // Handle bad states
        if (!elem.data("isAnimationInProgress")) {
            //console.log(`addAnotherCharacter called but animation was not in progress, resetting visible state`);
            elem.text(txt);
            return;
        }

        let speed = 60;
        if (i < txt.length) {
            const currentText = elem.text();
            elem.text(currentText + txt.charAt(i));
            setTimeout(addAnotherCharacter, speed, {data: {elem: elem, i: i + 1}})
        }
        else {
            //console.log(`Ending animation...`);
            elem.data("isAnimationInProgress", false);
            // Handle bad states
            elem.text(txt);
        }
    }

    $(window).scroll(function() {
        titles.each(function() {
            var hT = $(this).offset().top,
                hH = $(this).outerHeight(),
                wH = $(window).height(),
                wS = $(window).scrollTop();
            if (wS > (hT+hH-wH) && (hT > wS) && (wS+wH > hT+hH)){
                console.log(`Scrolled into view, needs animate? ${$(this).data("needsAnimate")}, isAnimInProgress? ${$(this).data("isAnimationInProgress")}`);
                if ($(this).data("needsAnimate") && !$(this).data("isAnimationInProgress")) {
                    $(this).data("isAnimationInProgress", true);
                    addAnotherCharacter({data: {elem: $(this), i: 2}})
                }
                $(this).data("needsAnimate", false);
            }
            else {
                $(this).data("needsAnimate", true);
                $(this).text("$ ")
            }
        })
    });
});

// Respond to arrow keys
document.onkeydown = function(e) {
    // Check whether we should eat this event
    // Is the mouse hovered over any of the input divs?
    for (const [snapshot_name, callbacks] of Object.entries(program_name_to_dispatch)) {
        // TODO(PT): Does anything bad happen if we call init twice? What if we don't call it?
        const computer_container_selector = `#computer_container_for_${snapshot_name}`;
        const program_container_selector = `#output_for_${snapshot_name}`;
        // Ref: https://stackoverflow.com/questions/8981463/detect-if-hovering-over-element-with-jquery
        if ($(computer_container_selector + ":hover").length === 0) {
            //console.log(`Mouse is not hovered over ${snapshot_name}`);
            continue;
        }
        // The mouse is hovered over this element
        console.log(`Mouse is hovered over ${snapshot_name}!`);
        // Inhibit the default action (scroll / move caret)
        e.preventDefault();

        const [init, entry, maybe_handle_input] = callbacks;
        if (!maybe_handle_input) {
            // Don't try to handle the arrow key here as we don't have an input handler for this program
            continue;
        }

        const keycode_to_direction_and_input_repr = {
            37: ["Left", "<left arrow>"],
            72: ["Left", "h"],
            38: ["Up", "<up arrow>"],
            75: ["Up", "k"],
            39: ["Right", "<right arrow>"],
            76: ["Right", "l"],
            40: ["Down", "<down arrow>"],
            74: ["Down", "j"],
        };
        const program_container = $(program_container_selector);
        if (!(e.which in keycode_to_direction_and_input_repr)) {
            program_container.text(`${program_container.text()}\nUnrecognized input!`);
        }
        else {
            const [direction, input_repr] = keycode_to_direction_and_input_repr[e.which];
            const new_text = maybe_handle_input(direction);
            program_container.text(`${program_container.text()}\n${input_repr}\n${new_text}`);
        }

        program_container.scrollTop(program_container.prop("scrollHeight"));
    }
}
