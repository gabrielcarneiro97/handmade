<div align="center">


  <img width="50%" src="./assets/notes-logo.png" alt="some handmade notes!">

  <h1><code>handmade</code></h1>

  <p>
    <strong>Type with your on handwritten letter.</strong>
  </p>
</div>


## 📓 Example

### Default Dictionary

Create an folder at your project root called `letters/default` with this [image](https://raw.githubusercontent.com/gabrielcarneiro97/handmade/master/letters/default/00%20abc.png) named `00 abc.png`.

Initialize the default dictionary, you need to do this just once. It'll create a file for each character.

```rust
use handmade::{text::*, text_img};

static LOREM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";

fn main() {
    text_img::update_images(None).unwrap();
}
```

Now you can write anything that you want.

```rust
use handmade::{text::*, text_img};

static LOREM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";

fn main() {
    let mut text = Text::new(&pages::DEFAULT);
    text.parse_to_png(LOREM, Some("example"));
}
```

You'll find the result at the `output` folder in a file named `example-1.png`. For each page, `handmade` will create a file and will put it on the output file. If `None` is used at `parse_to_png` function, the files identifier will be `page`.

This example above will generate this:

![img](./assets/example.png)

### Your own Dictionary

To use your own handwritten letter, first you'll need create a file just like the example below, you can found this example at the `assets` folder at the lib [repo](https://github.com/gabrielcarneiro97/handmade/tree/master/assets):

![img](./assets/00 abc.png)

All letters must be in one line, in this exactly order to it to work. Remove any imperfections between letters, this kind of thing can lead to unexpected results.

Then you'll need to create a new folder in `letters` folder with the desired name for the given dictionary, then put the letters file in it named `00 abc.png`.

The next step is to initialize the dictionary, it's done just like with `Default`, but you'll need to pass the dictionary name to `update_images` function.

For example, I've created a folder called `letters/myDictionary` with the `00 abc.png`, so this is the step-by-step usage:

```rust
use handmade::{text::*, text_img};

static LOREM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";

fn main() {
    let dic = "myDictionary"
    text_img::update_images(Some(dic)).unwrap();
}
```

To use, you'll need to create the `Text` struct with the function `Text::new_with_dic_name`, passing the dictionary name. Aside from that, the usage is the same.

```rust
use handmade::{text::*, text_img};

static LOREM : &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";

fn main() {
    let dic = "myDictionary";

    text_img::update_images(Some(dic)).unwrap();

    let mut text = Text::new_with_dic_name(&pages::DEFAULT, dic);
    text.parse_to_png(LOREM, Some("example"));
}
```

## 📘 Documentation

Under construction.

## 🎨 Prior Art

This library was inspired by the great [handwritten.js](https://github.com/alias-rahil/handwritten.js) library!
