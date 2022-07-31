import React from 'react';
import { WatermarkedImageWeb } from 'react-image';
import Canvas from '../Canvas/Canvas';

type WatermarkingPreviewProps = {
  image: WatermarkedImageWeb | undefined;
};

const WatermarkingPreview = ({ image }: WatermarkingPreviewProps) => {
  if (!image) {
    return <div className="WatermarkingPreview">Loading...</div>;
  }
  return (
    <div className="WatermarkingPreview">
      <p>Format: {image.get_format().toUpperCase()}</p>
      <p>
        Size: {image.get_width()}x{image.get_height()}
      </p>
      <div>
        <Canvas
          data={image.get_original()}
          width={image.get_width()}
          height={image.get_height()}
        />
        <Canvas
          data={image.get_watermarked()}
          width={image.get_width()}
          height={image.get_height()}
        />
      </div>
    </div>
  );
};

export default WatermarkingPreview;
