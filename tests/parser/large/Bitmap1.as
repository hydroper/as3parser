// Example provided by ActionScriptExamples.com.
const IMAGE_URL:String = "http://www.helpexamples.com/flash/images/logo.png";
 
var ldr:Loader = new Loader();
ldr.contentLoaderInfo.addEventListener(Event.COMPLETE, ldr_complete);
ldr.load(new URLRequest(IMAGE_URL));
 
var bitmap1:Bitmap;
var bitmap2:Bitmap;
var bitmap3:Bitmap;
var bitmap4:Bitmap;
 
function ldr_complete(evt:Event):void {
    var bmp:Bitmap = ldr.content as Bitmap;
 
    bitmap1 = new Bitmap(bmp.bitmapData);
    bitmap1.x = 100;
    bitmap1.y = 100;
    bitmap1.rotation = 0;
    addChild(bitmap1);
 
    bitmap2 = new Bitmap(bmp.bitmapData);
    bitmap2.x = 200;
    bitmap2.y = 100;
    bitmap2.rotation = 90;
    addChild(bitmap2);
 
    bitmap3 = new Bitmap(bmp.bitmapData);
    bitmap3.x = 300;
    bitmap3.y = 100;
    bitmap3.rotation = 180;
    addChild(bitmap3);
 
    bitmap4 = new Bitmap(bmp.bitmapData);
    bitmap4.x = 400;
    bitmap4.y = 100;
    bitmap4.rotation = 270;
    addChild(bitmap4);
}