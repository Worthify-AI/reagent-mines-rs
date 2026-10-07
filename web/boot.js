'use strict';
document.getElementById('glcanvas').addEventListener('contextmenu', event => event.preventDefault());
miniquad_add_plugin({register_plugin(importObject) {
  importObject.env.worthify_status = (phase, remaining, revealed) => {
    const status = document.getElementById('game-status');
    const text = `${['Ready','Playing','Won','Lost'][phase]}. ${remaining} flags left. ${revealed} safe squares revealed.`;
    if (status.textContent !== text) {
      status.textContent = text;
      Object.assign(status.dataset, {phase: String(phase), remaining: String(remaining), revealed: String(revealed)});
    }
  };
}});
load('reagent_mines_rs.wasm');
