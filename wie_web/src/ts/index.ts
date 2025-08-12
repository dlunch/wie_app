import { WieWeb } from "@pkg";

const main = () => {
  const file = document.getElementById("file") as HTMLInputElement;
  const button = document.getElementById("start") as HTMLButtonElement;

  button.addEventListener("click", () => {
    const selected_file = file.files[0];

    if (selected_file) {
      document.getElementById("file")?.remove();
      document.getElementById("start")?.remove();

      const original_console_error = console.error;
      console.error = (message: string, ...args) => {
        alert(message);
        original_console_error(message, ...args);
      };

      const reader = new FileReader();

      reader.onload = (e) => {
        const data = e.target?.result as ArrayBuffer;

        try {
          const canvas = document.getElementById("canvas") as HTMLCanvasElement;
          const wie_web = new WieWeb(
            selected_file.name,
            new Uint8Array(data),
            canvas
          );

          for (const button of document.querySelectorAll("button[data-key]")) {
            button.addEventListener("mousedown", (e) => {
              e.preventDefault();

              const key = (e.target as HTMLButtonElement).dataset.key;
              wie_web.key_down(key);
            });
            button.addEventListener("mouseup", (e) => {
              e.preventDefault();

              const key = (e.target as HTMLButtonElement).dataset.key;
              wie_web.key_up(key);
            });
          }
          const update = () => {
            try {
              wie_web.update();
            } catch (e) {
              wie_web.free();
              alert(e.message);
              throw e;
            }

            requestAnimationFrame(update);
          };

          requestAnimationFrame(update);
        } catch (e) {
          alert(e.message);
          throw e;
        }
      };

      reader.readAsArrayBuffer(selected_file);
    }
  });
};

if (document.readyState !== "loading") {
  main();
} else {
  document.addEventListener("DOMContentLoaded", () => main());
}
