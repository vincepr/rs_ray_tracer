// import("../pkg/index.js").catch(console.error);

import('../pkg/index.js')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const input_width = document.getElementById('input_width');
        const input_height = document.getElementById('input_height');
        const renderBtn = document.getElementById('render');

        renderBtn.addEventListener('click', () => {
            const width = parseInt(input_width.value) || 0;
            const height = parseInt(input_height.value) || 0;
            canvas.width = width;
            canvas.height = height;
            wasm.draw(ctx, width, height, "generatorstring");
        });

        // wasm.draw(ctx, 50, 50, "generatorstring");
        const drawImage = (url) => {
            const image = new Image();
            image.src = url;
            image.onload = () => {
                canvas.width = image.width;
                canvas.height = image.height;
               ctx.drawImage(image, 0, 0);
            }
        }
        drawImage("./out.png");
    })
    .catch(console.error);