window.onload = () => {
    let clock = document.getElementById('clock');
    clock.append(pair_new('hours'), pair_new('minutes'), pair_new('seconds'));
    setInterval(tick, 1000);
};

function pair_new(id) {
    let pair = document.createElement('div');
    pair.className = "pair";
    pair.id = id;
    pair.append(digit_new(), digit_new());
    return pair;
}

function digit_new() {
    let digit = document.createElement('div');
    digit.className = "digit";
    for (let i = 0; i < 24; ++i) {
        let cell = document.createElement('div');
        cell.className = "cell";
        let hand1 = document.createElement('div');
        hand1.className = 'hand';
        let hand2 = document.createElement('div');
        hand2.className = 'hand';
        cell.append(hand1, hand2);
        digit.append(cell);
    }
    return digit;
}

function tick() {
    let time = now();
    pair_tick(document.getElementById('hours'), time.hours);
    pair_tick(document.getElementById('minutes'), time.minutes);
    pair_tick(document.getElementById('seconds'), time.seconds);
}

function now() {
    const time = new Date().toLocaleTimeString("en-US", { 
        hour12: true,
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit"
    });
    const string = time.split(" ")[0];
    const [hours, minutes, seconds] = string.split(":");
    return {
        hours,
        minutes,
        seconds
    }
}

function pair_tick(elem, value) {
    let digits = elem.children;
    digit_tick(digits[0], value[0]);
    digit_tick(digits[1], value[1]);
}

function digit_tick(elem, value) {
    for (let i = 0; i < elem.children.length; ++i) {
        let cell = elem.children[i];
        const angle = angles[shapes[value][i]];
        cell.children[0].style.rotate = `${angle[0]}deg`;
        cell.children[1].style.rotate = `${angle[1]}deg`;
    }
}

const angles = {
  " ": [135, 135],
  "┘": [180, 270],
  "└": [0, 270],
  "┐": [90, 180],
  "┌": [0, 90],
  "-": [0, 180],
  "|": [90, 270]
}

const shapes = {
  "0": [
    "┌", "-", "-", "┐",
    "|", "┌", "┐", "|",
    "|", "|", "|", "|",
    "|", "|", "|", "|",
    "|", "└", "┘", "|",
    "└", "-", "-", "┘",
  ],

  "1": [
    "┌", "-", "┐", " ",
    "└", "┐", "|", " ",
    " ", "|", "|", " ",
    " ", "|", "|", " ",
    "┌", "┘", "└", "┐",
    "└", "-", "-", "┘",
  ],

  "2": [
    "┌", "-", "-", "┐",
    "└", "-", "┐", "|",
    "┌", "-", "┘", "|",
    "|", "┌", "-", "┘",
    "|", "└", "-", "┐",
    "└", "-", "-", "┘",
  ],

  "3": [
    "┌", "-", "-", "┐",
    "└", "-", "┐", "|",
    " ", "┌", "┘", "|",
    " ", "└", "┐", "|",
    "┌", "-", "┘", "|",
    "└", "-", "-", "┘",
  ],

  "4": [
    "┌", "┐", "┌", "┐",
    "|", "|", "|", "|",
    "|", "└", "┘", "|",
    "└", "-", "┐", "|",
    " ", " ", "|", "|",
    " ", " ", "└", "┘",
  ],

  "5": [
    "┌", "-", "-", "┐",
    "|", "┌", "-", "┘",
    "|", "└", "-", "┐",
    "└", "-", "┐", "|",
    "┌", "-", "┘", "|",
    "└", "-", "-", "┘",
  ],

  "6": [
    "┌", "-", "-", "┐",
    "|", "┌", "-", "┘",
    "|", "└", "-", "┐",
    "|", "┌", "┐", "|",
    "|", "└", "┘", "|",
    "└", "-", "-", "┘",
  ],

  "7": [
    "┌", "-", "-", "┐",
    "└", "-", "┐", "|",
    " ", " ", "|", "|",
    " ", " ", "|", "|",
    " ", " ", "|", "|",
    " ", " ", "└", "┘",
  ],

  "8": [
    "┌", "-", "-", "┐",
    "|", "┌", "┐", "|",
    "|", "└", "┘", "|",
    "|", "┌", "┐", "|",
    "|", "└", "┘", "|",
    "└", "-", "-", "┘",
  ],

  "9": [
    "┌", "-", "-", "┐",
    "|", "┌", "┐", "|",
    "|", "└", "┘", "|",
    "└", "-", "┐", "|",
    "┌", "-", "┘", "|",
    "└", "-", "-", "┘",
  ]
}

window.wallpaperPropertyListener = {
    applyUserProperties: function(properties) {
        let r = document.querySelector(':root');
        // if (properties.handcolor) {
        //     // Convert the custom color to 0 - 255 range for CSS usage
        //     let handcolor = properties.handcolor.value.split(' ');
        //     handcolor = handcolor.map(function(c) {
        //         return Math.ceil(c * 255);
        //     });
        //     let handcolorAsCSS = 'rgb(' + handcolor + ')';
        //     r.style.setProperty('--hand-color', handcolorAsCSS);
        // }

        if (properties.schemecolor) {
            // Convert the custom color to 0 - 255 range for CSS usage
            let handcolor = properties.schemecolor.value.split(' ');
            handcolor = handcolor.map(function(c) {
                return Math.ceil(c * 255);
            });
            let handcolorAsCSS = 'rgb(' + handcolor + ')';
            r.style.setProperty('--hand-color', handcolorAsCSS);
        }

        if (properties.backgroundimage) {
            document.getElementById('clock').style.backgroundImage = 
                "url('file:///" + properties.backgroundimage.value + "')";
        }

        if (properties.transitionDuration) {
            r.style.setProperty('--transition', properties.transitionDuration.value + "s");
        }
        if (properties.scale) {
            r.style.setProperty('--root-font-size', properties.scale.value + "px");
        }

        // console.log(properties);
    },
};
