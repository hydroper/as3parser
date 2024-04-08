// Example from www.airsdk.dev/reference
package {
    import flash.display.Bitmap;
    import flash.display.BitmapData;
    import flash.display.Loader;
    import flash.display.Sprite;
    import flash.events.*;
    import flash.geom.Point;
    import flash.geom.Rectangle;
    import flash.net.URLRequest;

    public class BitmapExample extends Sprite {
        private var url:String = "Image.gif";
        private var size:uint = 80;

        public function BitmapExample() {
            configureAssets();
        }

        private function configureAssets():void {
            var loader:Loader = new Loader();
            loader.contentLoaderInfo.addEventListener(Event.COMPLETE, completeHandler);
            loader.contentLoaderInfo.addEventListener(IOErrorEvent.IO_ERROR, ioErrorHandler);

            var request:URLRequest = new URLRequest(url);
            loader.x = size * numChildren;
            loader.load(request);
            addChild(loader);
        }

        private function duplicateImage(original:Bitmap):Bitmap {
            var image:Bitmap = new Bitmap(original.bitmapData.clone());
            image.x = size * numChildren;
            addChild(image);
            return image;
        }

        private function completeHandler(event:Event):void {
            var loader:Loader = Loader(event.target.loader);
            var image:Bitmap = Bitmap(loader.content);

            var duplicate:Bitmap = duplicateImage(image);
            var bitmapData:BitmapData = duplicate.bitmapData;
            var sourceRect:Rectangle = new Rectangle(0, 0, bitmapData.width, bitmapData.height);
            var destPoint:Point = new Point();
            var operation:String = ">=";
            var threshold:uint = 0xCCCCCCCC;
            var color:uint = 0xFFFFFF00;
            var mask:uint = 0x000000FF;
            var copySource:Boolean = true;

            bitmapData.threshold(bitmapData,
                                 sourceRect,
                                 destPoint,
                                 operation,
                                 threshold,
                                 color,
                                 mask,
                                 copySource);
        }
        
        private function ioErrorHandler(event:IOErrorEvent):void {
            trace("Unable to load image: " + url);
        }
    }
}