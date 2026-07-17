export function polar(radius: number, angle: number) {
  const rad = (angle * Math.PI) / 180;

  return {
    x: Math.cos(rad) * radius,

    y: Math.sin(rad) * radius,
  };
}
