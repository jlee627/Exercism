//
// This is only a SKELETON file for the 'Space Age' exercise. It's been provided as a
// convenience to get you started writing code faster.
//
const earthYearSeconds = 31557600;

const planetRate = {
  'earth': 1 * earthYearSeconds,
  'mercury': 0.2408467 * earthYearSeconds,
  'venus': 0.61519726 * earthYearSeconds,
  'mars': 1.8808158 * earthYearSeconds,
  'jupiter': 11.862615 * earthYearSeconds,
  'saturn': 29.447498 * earthYearSeconds,
  'uranus': 84.016846 * earthYearSeconds,
  'neptune': 164.79132 * earthYearSeconds,
}

export const age = (planet, seconds) => {
  return parseFloat((seconds/planetRate[planet]).toFixed(2));
};

// Things to remember: 
// .toFixed() returns a string, so you'll need to convert it to a float.