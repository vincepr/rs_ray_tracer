import('../pkg/index.js')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const input_width = document.getElementById('input_width');
        const input_height = document.getElementById('input_height');
        const renderBtn = document.getElementById('render');
        const time_result = document.getElementById('time_result');

        renderBtn.addEventListener('click', () => {
            const width = parseInt(input_width.value) || 0;
            const height = parseInt(input_height.value) || 0;
            canvas.width = width;
            canvas.height = height;
            const start = new Date();
            wasm.draw(ctx, width, height, "generatorstring");
            const time_ms = new Date().getTime() - start.getTime();
            time_result.innerHTML = `it took ${Math.round(time_ms/1000)}s to render`;
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
        drawImage("./example.png");
    })
    .catch(console.error);