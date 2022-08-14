import React from 'react';
import Canvas from '../Canvas/Canvas';
import { WatermarkingProcess } from '../Content/image';

type WatermarkingPreviewProps = {
  watermarkingProcess: WatermarkingProcess | undefined;
};

const WatermarkingPreview = ({ watermarkingProcess }: WatermarkingPreviewProps) => {
  if (!watermarkingProcess) {
    return <div className="WatermarkingPreview">Loading...</div>;
  }
  return (
    <div className="WatermarkingPreview">
      <p>Format: {watermarkingProcess.sourceFormat.toUpperCase()}</p>
      <p>
        Size: {watermarkingProcess.source.width}x{watermarkingProcess.source.height}
      </p>
      <div>
        <Canvas
          data={watermarkingProcess.source.data}
          width={watermarkingProcess.source.width}
          height={watermarkingProcess.source.height}
        />
        {
          <Canvas
            data={watermarkingProcess.watermarked.data}
            width={watermarkingProcess.watermarked.width}
            height={watermarkingProcess.watermarked.height}
          />
        }
      </div>
    </div>
  );
};

export default WatermarkingPreview;
