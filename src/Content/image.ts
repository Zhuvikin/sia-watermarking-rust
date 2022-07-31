export type WatermarkedImage = {
  width: number;
  height: number;
  format: string;
  original: Uint8Array;
  watermarked: Uint8Array;
};
