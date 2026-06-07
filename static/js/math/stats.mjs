export function mean(values) {
  if (!values.length) return 0;
  return values.reduce((sum, value) => sum + value, 0) / values.length;
}

export function standardDeviation(values, precomputedMean = null) {
  if (!values.length) return 0;

  const avg = precomputedMean ?? mean(values);
  const variance = values.reduce((sum, value) => {
    const delta = value - avg;
    return sum + delta * delta;
  }, 0) / values.length;

  return Math.sqrt(variance);
}

export function coefficientOfVariation(values) {
  if (!values.length) return 0;

  const avg = mean(values);
  if (avg === 0) return 0;

  return standardDeviation(values, avg) / avg;
}
