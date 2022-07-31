import React, { MutableRefObject, useEffect, useRef } from 'react';

type CanvasProps = {
  data: Uint8Array;
  width: number;
  height: number;
};

const Canvas = ({
  data,
  width,
  height,
  ...rest
}: CanvasProps &
  JSX.IntrinsicAttributes &
  React.ClassAttributes<HTMLCanvasElement> &
  React.CanvasHTMLAttributes<HTMLCanvasElement>) => {
  const canvasRef = useRef() as MutableRefObject<HTMLCanvasElement>;
  useEffect(() => {
    const canvas = canvasRef.current;
    const context = canvas.getContext('2d');
    const imageData = context!.createImageData(width, height);

    imageData.data.set(data, 0);
    context!.putImageData(imageData, 0, 0);
  }, [data, width, height]);

  return <canvas width={width} height={height} ref={canvasRef} {...rest} />;
};

export default Canvas;
