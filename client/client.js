const graphics = {
    X: ` <svg class="X"
    width="100%"
    height="100%"
    viewBox="0 0 250 250"
    version="1.1">
    
       <path
          style="fill:none;;stroke:#000000;stroke-width:34;stroke-linecap:round;stroke-opacity:1;paint-order:fill markers stroke"
          d="m 45.959847,51.669008 c 39.154894,32.918149 58.308583,61.330072 93.242213,88.208572 26.40273,25.34475 41.07327,48.325 75.74432,64.71825"
          class="leftstroke"/>
       <path
          style="fill:none;fill-opacity:1;stroke:#000000;stroke-width:34;stroke-linecap:round;paint-order:fill markers stroke"
          d="M 200.42722,42.957289 C 165.01341,79.027137 148.71863,108.96495 116.91457,138.65858 93.297602,166.12452 68.454389,179.72525 48.494134,209.4724"
          class="rightstroke"/>
    
    </svg>`,
    O: `<svg class="O"
    width="100%"
    height="100%"
    viewBox="30 290 250 250"
    version="1.1">
       <path
          style="fill:none;fill-opacity:1;stroke:#000000;stroke-width:34.0157;stroke-linecap:round;stroke-opacity:1;paint-order:fill markers stroke"
          d="m 84.853452,379.64605 c 13.914892,-22.66004 30.062118,-40.94502 81.838728,-43.80097 20.67531,3.6223 27.47275,5.42895 38.85799,18.19863 26.22457,30.97782 25.24538,52.6686 18.63475,84.97896 -4.76582,14.40865 -12.33192,21.84728 -19.36624,30.5973 -5.25832,6.55555 -13.60397,14.59696 -18.28738,17.55588 -4.38594,2.74121 -10.37194,5.9542 -15.36141,7.31496 -12.18955,1.71918 -24.35943,2.28081 -35.84326,1.46299 -18.11353,-3.6841 -29.88269,-8.62899 -43.88972,-24.13935 -4.544369,-6.57769 -6.956006,-14.9653 -8.046448,-22.67635 -0.833799,-15.23925 -2.768637,-24.87956 -1.645864,-38.76925"
          class="o-path" />
    
    
    </svg>`,
    diag1: `<svg class="X"
    width="100%"
    height="100%"
    viewBox="0 0 250 250"
    version="1.1">
    
       <path
          style="fill:none;;stroke:#000000;stroke-width:34;stroke-linecap:round;stroke-opacity:1;paint-order:fill markers stroke"
          d="m 45.959847,51.669008 c 39.154894,32.918149 58.308583,61.330072 93.242213,88.208572 26.40273,25.34475 41.07327,48.325 75.74432,64.71825"
          class="slowleftstroke"/>
    
    </svg>`,
    diag2: `<svg class="X"
    width="100%"
    height="100%"
    viewBox="0 0 250 250"
    version="1.1">
    
       <path
          style="fill:none;fill-opacity:1;stroke:#000000;stroke-width:34;stroke-linecap:round;paint-order:fill markers stroke"
          d="M 200.42722,42.957289 C 165.01341,79.027137 148.71863,108.96495 116.91457,138.65858 93.297602,166.12452 68.454389,179.72525 48.494134,209.4724"
          class="slowrightstroke"/>
    
    </svg>`,
    sidestroke: `<svg
    width="100%"
    height="100%"
    viewBox="20 -90 290 290"
    version="1.1">
    
       <path transform="rotate(-45 50 50)"
          style="fill:none;;stroke:#000000;stroke-width:34;stroke-linecap:round;stroke-opacity:1;paint-order:fill markers stroke"
          d="m 45.959847,51.669008 c 39.154894,32.918149 58.308583,61.330072 93.242213,88.208572 26.40273,25.34475 41.07327,48.325 75.74432,64.71825"
          class="sidestroke"/>
    
    
    </svg>`,
    sidestrokeU: `<svg
    width="100%"
    height="100%"
    viewBox="20 0 290 290"
    version="1.1">
    
       <path transform="rotate(-45 50 50)"
          style="fill:none;;stroke:#000000;stroke-width:34;stroke-linecap:round;stroke-opacity:1;paint-order:fill markers stroke"
          d="m 45.959847,51.669008 c 39.154894,32.918149 58.308583,61.330072 93.242213,88.208572 26.40273,25.34475 41.07327,48.325 75.74432,64.71825"
          class="sidestroke"/>
    
    
    </svg>`,
    sidestrokeD: `<svg
    width="100%"
    height="100%"
    viewBox="20 -180 290 290"
    version="1.1">
    
       <path transform="rotate(-45 50 50)"
          style="fill:none;;stroke:#000000;stroke-width:34;stroke-linecap:round;stroke-opacity:1;paint-order:fill markers stroke"
          d="m 45.959847,51.669008 c 39.154894,32.918149 58.308583,61.330072 93.242213,88.208572 26.40273,25.34475 41.07327,48.325 75.74432,64.71825"
          class="sidestroke"/>
    
    
    </svg>`,
    
    downstroke: `<svg
    width="100%"
    height="100%"
    viewBox="0 -90 290 290"
    version="1.1">
    
       <path transform="rotate(-45 50 50)" 
          style="fill:none;fill-opacity:1;stroke:#000000;stroke-width:34;stroke-linecap:round;paint-order:fill markers stroke"
          d="M 200.42722,42.957289 C 165.01341,79.027137 148.71863,108.96495 116.91457,138.65858 93.297602,166.12452 68.454389,179.72525 48.494134,209.4724"
          class="downstroke"/>
    
    </svg>`,
    
    downstrokeR: `<svg
    width="100%"
    height="100%"
    viewBox="-80 -90 290 290"
    version="1.1">
    
       <path transform="rotate(-45 50 50)" 
          style="fill:none;fill-opacity:1;stroke:#000000;stroke-width:34;stroke-linecap:round;paint-order:fill markers stroke"
          d="M 200.42722,42.957289 C 165.01341,79.027137 148.71863,108.96495 116.91457,138.65858 93.297602,166.12452 68.454389,179.72525 48.494134,209.4724"
          class="downstroke"/>
    
    </svg>`,
    downstrokeL: `<svg
    width="100%"
    height="100%"
    viewBox="100 -90 290 290"
    version="1.1">
    
       <path transform="rotate(-45 50 50)" 
          style="fill:none;fill-opacity:1;stroke:#000000;stroke-width:34;stroke-linecap:round;paint-order:fill markers stroke"
          d="M 200.42722,42.957289 C 165.01341,79.027137 148.71863,108.96495 116.91457,138.65858 93.297602,166.12452 68.454389,179.72525 48.494134,209.4724"
          class="downstroke"/>
    
    </svg>`,
    scribble: `<svg
    width = "100%"
    height = "100%"
    viewBox = "-30 -30 300 300">
    <path
           style="fill:none;fill-opacity:1;stroke-linecap:round;stroke-opacity:1;paint-order:fill markers stroke"
           d="M 0,29.25981 C 43.3248,12.441666 26.666442,14.741564 67.663312,9.143691 c 0.60958,0 -1.3977,-0.4310377 -1.828737,0 -3.973973,3.973972 -6.998458,8.827196 -10.97243,12.801167 -38.423144,40.198429 -17.521832,19.068477 -43.778784,46.577359 -2.6066671,3.689775 -8.8587448,9.847117 -9.2546228,13.771 0,0.792456 -2.25544377,3.657478 -1.8287382,3.657478 34.060545,-13.497713 72.786199,-28.071146 102.40934,-42.060977 10.0874,-4.227013 19.14231,-9.455907 29.25981,-14.629908 0.94793,-0.473965 5.48622,-5.01746 5.48622,-3.657475 0,3.047898 -4.80336,3.759493 -7.31496,5.486215 -7.94236,5.460373 -15.97876,10.78967 -23.77359,16.458643 -25.937183,18.863402 -66.221157,43.185992 -85.9507,69.492057 -2.472786,3.29704 -4.199124,7.11115 -5.486214,10.97242 -0.744711,2.23414 1.573134,9.3993 0,10.97243 -0.431038,0.43104 -1.828739,-0.60958 -1.828739,0 0,0.31387 9.156913,-1.10253 20.116121,-5.48621 17.17213,-6.86885 34.83545,-15.19924 51.20467,-23.7736 23.412742,-12.263817 45.831982,-26.573467 69.492052,-38.4035 17.2961,-6.834858 32.36346,-14.045107 47.54719,-21.94486 5.15088,-2.838426 6.06994,-3.215752 18.28738,-5.486212 6.3956,0.316177 -13.07087,7.861101 -14.6299,9.14369 -7.52625,6.020994 -16.19483,10.503897 -23.7736,16.458644 -25.8863,20.339233 -51.09379,41.606528 -76.807,62.177098 -5.29737,4.23789 -9.58593,10.09032 -14.629908,14.6299 -0.0931,0.0838 -19.6159,15.37619 -21.94486,18.28739 -7.293995,11.14203 -11.185027,15.32829 -14.629905,21.94485 -0.44471,0.88943 -45.4589655,57.94517 -9.14369,21.94486 6.776127,-6.71736 17.068223,-8.53411 25.602335,-12.80116 17.064023,-8.53202 34.140648,-17.07033 51.204668,-25.60234 16.54291,-8.27146 32.80341,-17.31607 49.37593,-25.60233 12.6454,-6.3227 24.50926,-12.87682 36.57476,-20.11612 5.17679,-3.10608 11.37566,-3.9263 16.45865,-7.31495 3.17258,-2.44606 5.6037,-3.71623 9.14369,-5.48622 0.54523,-0.27261 1.82874,0.60958 1.82874,0 0,-3.4483 -4.87664,4.87664 -7.31496,7.31495 -6.56799,6.568 -12.45616,12.8271 -18.28738,20.11612 -18.91969,23.64961 -42.67524,42.67525 -64.00583,64.00584 -6.95461,6.9546 -15.8098,17.52074 -18.28739,27.43107 0.51546,3.08388 -34.246119,20.77866 -1.82873,7.31496 2.71226,2.71226 11.67227,-4.88414 12.80116,-5.48622 7.21621,-3.84864 14.68448,-7.20779 21.94486,-10.97243 91.01608,-48.5419 -8.08594,1.84519 40.23224,-18.28738 1.63833,-0.68264 38.95812,-22.66437 40.23224,-20.11612 3.58971,7.17942 -15.62371,34.06998 -18.28738,42.06098 -1.15404,7.3023 -3.58714,10.45293 -3.65748,14.6299"
           class="scribble"/>
        </svg>`
    
}

let addmove = (fieldindex) => {
        //console.log(fieldindex);
        fetch('./'.concat(fieldindex), { method: 'POST' })
        .then((response) => {        
            console.log(response);                   
        });
}

let updatefields = (gamestate) => {
   fields.forEach((field, i) => {
      //console.log(field.getAttribute("state"), " ==? ", data[i]);
        if (field.getAttribute("state") !== gamestate[i]) {
            console.log("Field number ", i, "updating to", gamestate[i]);
            field.setAttribute("state", gamestate[i]);
            if (gamestate[i] === "X") {
               field.innerHTML = graphics.X;
            } else if (gamestate[i] === "O") {
               field.innerHTML = graphics.O;
            } else {
               field.innerHTML = "";
            }
        }
    });
}

let showvictory = (condition) => {
   console.log("Victory: " + condition);
   let overlay = document.querySelector(".overlay");
   overlay.style.display = "block";
   switch (condition) {
       case 0:
           overlay.innerHTML = graphics.sidestrokeU;
           break;
       case 1:
           overlay.innerHTML = graphics.sidestroke;
           break;
       case 2:
           overlay.innerHTML = graphics.sidestrokeD;
           break;
       case 3:
           overlay.innerHTML = graphics.downstrokeL;
           break;
       case 4:
           overlay.innerHTML = graphics.downstroke;
           break;
       case 5:
           overlay.innerHTML = graphics.downstrokeR;
           break;
       case 6:
           overlay.innerHTML = graphics.diag1;
           break;
       case 7:
           overlay.innerHTML = graphics.diag2;
           break;
   }
}

let messages = document.getElementById("notifications");
let board = document.getElementById("board");
let fields = [];

for (let i = 0 ; i <= 8 ; i++) {
    let newelem = document.createElement("div");
    newelem.className = "field";
    newelem.setAttribute("state", "Empty");
    newelem.addEventListener("click", () => {
        addmove(i);
    });
    fields.push(newelem);
    board.append(newelem);
}

let events = new EventSource("./events");
events.onmessage = (event) => {
    let data = JSON.parse(event.data);
    let gamestate = data.gamestate;
    let nextup = data.nextup;
    let outcome = data.outcome;
    console.log(data);
    updatefields(gamestate);
    if (outcome) {
      showvictory(outcome[1]);
    }
}

        

