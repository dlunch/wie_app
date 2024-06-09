import { WieWeb, WieWebBridge } from "./pkg";

class MidiPlayer {
  public note_on(channel_id: number, note: number, velocity: number) {
    console.log(`note_on: ${channel_id} ${note} ${velocity}`);
  }

  public note_off(channel_id: number, note: number, velocity: number) {
    console.log(`note_off: ${channel_id} ${note}`);
  }

  public control_change(channel_id: number, control: number, value: number) {
    console.log(`control_change: ${channel_id} ${control} ${value}`);
  }

  public program_change(channel_id: number, program: number) {
    console.log(`program_change: ${channel_id} ${program}`);
  }
}

const main = () => {
  const file = document.getElementById("file") as HTMLInputElement;
  const button = document.getElementById("start") as HTMLButtonElement;

  button.addEventListener("click", () => {
    let selected_file = file.files[0];

    if (selected_file) {
      let reader = new FileReader();

      reader.onload = (e) => {
        let data = e.target?.result as ArrayBuffer;

        try {
          const midi_player = new MidiPlayer();
          const bridge = new WieWebBridge(
            (channel_id: number, note: number, velocity: number) =>
              midi_player.note_on(channel_id, note, velocity),
            (channel_id: number, note: number, velocity: number) =>
              midi_player.note_off(channel_id, note, velocity),
            (channel_id: number, control: number, value: number) =>
              midi_player.control_change(channel_id, control, value),
            (channel_id: number, program: number) =>
              midi_player.program_change(channel_id, program)
          );

          const canvas = document.getElementById("canvas") as HTMLCanvasElement;
          const wie_web = new WieWeb(
            selected_file.name,
            new Uint8Array(data),
            canvas,
            bridge
          );

          for (const button of document.querySelectorAll("button[data-key]")) {
            button.addEventListener("mousedown", (e) => {
              const key = (e.target as HTMLButtonElement).dataset.key;
              wie_web.send_key(key);
            });
          }
          let update = () => {
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
