class MidiPlayer {
  constructor() {}

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
