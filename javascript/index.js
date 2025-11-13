// MIT License

// Copyright (c) 2025 benbinn

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

let tick_interval_s = 1;

window.onload = () => {
    let clock = document.getElementById('clock');
    clock.append(pair_new('hours'), pair_new('minutes'), pair_new('seconds'));
    tick.deviation = 0;
    tick.next_tick = new Date();
    tick();
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
    let now = new Date();
    pair_tick(document.getElementById('hours'),   now.getHours());
    pair_tick(document.getElementById('minutes'), now.getMinutes());
    pair_tick(document.getElementById('seconds'), now.getSeconds());

    // high percision
    // feels like phase lock loop
    let offs = now - tick.next_tick;
    if (Math.abs(offs) > 10) {
        let p = 0.4;
        tick.deviation = tick.deviation * p + offs * (1-p);
    }

    // next_tick = now+interval
    // ticks at: 
    //  a) next_tick
    //  b) second border, i.e. start of next second or start 
    //     of next_tick's second, whichever comes *later*
    if (offs >= 0) {
        let next_tick = tick.next_tick.getTime() + tick_interval_s * 1000;
        let next_second = Math.ceil(now/1e3)*1e3;
        let next_tick_second = Math.floor(next_tick/1e3)*1e3;
        let second_border = Math.max(next_second, next_tick_second);
        tick.next_tick.setTime( Math.min(next_tick, second_border) );
    }

    // console.log("now: " + now.getSeconds() + "." + now.getMilliseconds() + "s", 
    //     "offs: " + offs + "ms",
    //     "devi: " + tick.deviation + "ms",
    //     "next: " + tick.next_tick.getSeconds() + "." + tick.next_tick.getMilliseconds() + "s");
    
    setTimeout(tick, tick.next_tick - now - tick.deviation);
}

function pair_tick(elem, value) {
    let digits = elem.children;
    digit_tick(digits[0], Math.floor(value/10));
    digit_tick(digits[1], value % 10);
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
        if (properties.handColor) {
            // Convert the custom color to 0 - 255 range for CSS usage
            let handcolor = properties.handColor.value.split(' ');
            handcolor = handcolor.map(function(c) {
                return Math.ceil(c * 255);
            });
            r.style.setProperty('--hand-color-rgb', handcolor);
        }
        if (properties.cellBGColor) {
            // Convert the custom color to 0 - 255 range for CSS usage
            let rgb = properties.cellBGColor.value.split(' ');
            rgb = rgb.map(function(c) {
                return Math.ceil(c * 255);
            });
            r.style.setProperty('--cell-background-rgb', rgb);
        }
        if (properties.schemecolor) {
            // Convert the custom color to 0 - 255 range for CSS usage
            let rgb = properties.schemecolor.value.split(' ');
            rgb = rgb.map(function(c) {
                return Math.ceil(c * 255);
            });
            document.getElementById('clock').style.backgroundColor = 
                "rgb(" + rgb + ")";
        }
        if (properties.backgroundimage) {
            let clock = document.getElementById('clock');
            clock.style.backgroundImage = 
                "url('file:///" + properties.backgroundimage.value + "')";
            clock.style.backgroundRepeat = "no-repeat";
            clock.style.backgroundSize = "contain";
            clock.style.backgroundPosition="center";
        }
        if (properties.transitionDuration) {
            r.style.setProperty('--transition', properties.transitionDuration.value + "s");
        }
        if (properties.scale) {
            r.style.setProperty('--root-font-size', properties.scale.value + "px");
        }
        if (properties.cellOpacity) {
            r.style.setProperty('--cell-opacity', properties.cellOpacity.value);
        }
        if (properties.handOpacity) {
            r.style.setProperty('--hand-color-opacity', properties.handOpacity.value);
        }
        if (properties.refreshTime) {
            tick_interval_s = properties.refreshTime.value;
        }
        // console.log(properties);
    },
};
