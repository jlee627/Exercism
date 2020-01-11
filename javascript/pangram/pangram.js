//
// This is only a SKELETON file for the 'Pangram' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const isPangram = (possiblePangram) => {
  return new Set(possiblePangram.toLowerCase().match(/[a-z]/g)).size === 26
};
