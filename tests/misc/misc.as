package {
    import flash.display.Sprite;
    import flash.text.TextField;
    import flash.text.TextFieldType;

    public class TextField_alwaysShowSelection extends Sprite {
        public function TextField_alwaysShowSelection() {
            var label1:TextField = createCustomTextField(0, 20, 200, 20);
            label1.text = "This text is selected.";
            label1.setSelection(0, 9);
            label1.alwaysShowSelection = true;

            var label2:TextField = createCustomTextField(0, 50, 200, 20);
            label2.text = "Drag to select some of this text.";
        }

        private function createCustomTextField(x:Number, y:Number, width:Number, height:Number):TextField {
            var result:TextField = new TextField();
            result.x = x; result.y = y;
            result.width = width; result.height = height;
            addChild(result);
            return result;
        }
    }
}