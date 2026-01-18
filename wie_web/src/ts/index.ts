import { WieWeb } from "@pkg";
import "../css/style.css";

const TUTORIAL_STORAGE_KEY = "wie_tutorial_dismissed";

const initTutorial = () => {
  const overlay = document.getElementById("tutorial-overlay");
  const closeButton = document.getElementById("close-tutorial");
  const dontShowCheckbox = document.getElementById("dont-show-again") as HTMLInputElement;

  if (!overlay || !closeButton || !dontShowCheckbox) return;

  if (localStorage.getItem(TUTORIAL_STORAGE_KEY) === "true") {
    overlay.classList.add("hidden");
    return;
  }

  closeButton.addEventListener("click", () => {
    if (dontShowCheckbox.checked) {
      localStorage.setItem(TUTORIAL_STORAGE_KEY, "true");
    }
    overlay.classList.add("hidden");
  });

  overlay.addEventListener("click", (e) => {
    if (e.target === overlay) {
      if (dontShowCheckbox.checked) {
        localStorage.setItem(TUTORIAL_STORAGE_KEY, "true");
      }
      overlay.classList.add("hidden");
    }
  });
};

const key_map = {
  Digit1: "1",
  Digit2: "2",
  Digit3: "3",
  KeyQ: "4",
  KeyW: "5",
  KeyE: "6",
  KeyA: "7",
  KeyS: "8",
  KeyD: "9",
  KeyZ: "*",
  KeyX: "0",
  KeyC: "#",
  Backspace: "CLR",
  ArrowUp: "UP",
  ArrowLeft: "LEFT",
  ArrowRight: "RIGHT",
  ArrowDown: "DOWN",
  Space: "OK",
};

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
          document.addEventListener("keydown", (e) => {
            if (key_map[e.code]) {
              e.preventDefault();
              wie_web.key_down(key_map[e.code]);
            }
          });
          document.addEventListener("keyup", (e) => {
            if (key_map[e.code]) {
              e.preventDefault();
              wie_web.key_up(key_map[e.code]);
            }
          });
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
  initTutorial();
  main();
} else {
  document.addEventListener("DOMContentLoaded", () => {
    initTutorial();
    main();
  });
}
