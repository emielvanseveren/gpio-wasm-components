// Imports are the `wit package` followed by the interface
import { gpioInit } from "thesis:gpio/types";

const Mode = Object.freeze({
  INPUT: 0,
  OUTPUT: 1,
});

export function start() {
  gpioInit(1000, 0, Mode.INPUT);
  return 0;
}

export function hostToGuest() {
  console.log("host-to-guest triggered");
}
