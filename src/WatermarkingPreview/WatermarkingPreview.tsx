import React from 'react';
import Canvas from '../Canvas/Canvas';
import { WatermarkedImage } from '../Content/image';

type WatermarkingPreviewProps = {
  image: WatermarkedImage | undefined;
};

const WatermarkingPreview = ({ image }: WatermarkingPreviewProps) => {
  if (!image) {
    return <div className="WatermarkingPreview">Loading...</div>;
  }
  return (
    <div className="WatermarkingPreview">
      <p>Format: {image.format.toUpperCase()}</p>
      <p>
        Size: {image.width}x{image.height}
      </p>
      <div>
        <Canvas
          data={image.original}
          width={image.width}
          height={image.height}
        />
        {
          <Canvas
            data={image.watermarked}
            width={image.width}
            height={image.height}
          />
        }
      </div>
    </div>
  );
};

export default WatermarkingPreview;
