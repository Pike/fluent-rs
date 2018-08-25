extern crate fluent;

use std::collections::HashMap;

use self::fluent::context::FluentBundle;
use self::fluent::types::FluentValue;

#[test]
fn select_expression_without_selector() {
    let mut bundle = FluentBundle::new(&["x-testing"]);

    bundle.add_messages(
        "
foo =
    {
       *[nominative] Foo
        [genitive] Foo's
    }

bar =
    {
        [genitive] Bar's
       *[nominative] Bar
    }
",
    );

    let value = bundle.format("foo", None);
    assert_eq!(value, Some("Foo".to_string()));

    let value = bundle.format("bar", None);
    assert_eq!(value, Some("Bar".to_string()));
}

#[test]
fn select_expression_string_selector() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
foo =
    { \"genitive\" ->
       *[nominative] Foo
        [genitive] Foo's
    }

bar =
    { \"missing\" ->
       *[nominative] Bar
        [genitive] Bar's
    }
",
    );

    let value = bundle.format("foo", None);
    assert_eq!(value, Some("Foo's".to_string()));

    let value = bundle.format("bar", None);
    assert_eq!(value, Some("Bar".to_string()));
}

#[test]
fn select_expression_number_selector() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
foo =
    { 3 ->
       *[1] Foo 1
        [3] Foo 3
    }

bar =
    { 3 ->
       *[1] Bar 1
        [2] Bar 2
    }

baz =
    { 3.14 ->
       *[1] Baz 1
        [3] Baz 3
        [3.14] Baz Pi
    }
",
    );

    let value = bundle.format("foo", None);
    assert_eq!(value, Some("Foo 3".to_string()));

    let value = bundle.format("bar", None);
    assert_eq!(value, Some("Bar 1".to_string()));

    let value = bundle.format("baz", None);
    assert_eq!(value, Some("Baz Pi".to_string()));
}

#[test]
fn select_expression_plurals() {
    let mut bundle = FluentBundle::new(&["en"]);
    bundle.add_messages(
        "
foo =
    { 3 ->
        [one] Foo One
        [3] Foo 3
       *[other] Foo Other
    }

bar =
    { 1 ->
        [one] Bar One
        [2] Bar 2
       *[other] Bar Other
    }

baz =
    { \"one\" ->
        [1] Bar One
        [3] Bar 3
       *[other] Bar Other
    }
",
    );

    let value = bundle.format("foo", None);
    assert_eq!(value, Some("Foo 3".to_string()));

    let value = bundle.format("bar", None);
    assert_eq!(value, Some("Bar One".to_string()));

    let value = bundle.format("baz", None);
    assert_eq!(value, Some("Bar Other".to_string()));
}

#[test]
fn select_expression_external_argument_selector() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
foo-hit =
    { $str ->
       *[foo] Foo
        [qux] Qux
    }

foo-miss =
    { $str ->
       *[foo] Foo
        [bar] Bar
    }

foo-unknown =
    { $unknown ->
       *[foo] Foo
        [bar] Bar
    }

bar-hit =
    { $int ->
       *[1] Bar 1
        [3] Bar 3
    }

bar-miss =
    { $int ->
       *[1] Bar 1
        [2] Bar 2
    }

bar-unknown =
    { $unknown ->
       *[1] Bar 1
        [2] Bar 2
    }

baz-hit =
    { $float ->
       *[1] Baz 1
        [2.72] Baz E
    }

baz-miss =
    { $float ->
       *[1] Baz 1
        [2] Baz 2
    }

baz-unknown =
    { $unknown ->
       *[1] Baz 1
        [2] Baz 2
    }
",
    );

    let mut args = HashMap::new();
    args.insert("str", FluentValue::from("qux"));
    args.insert("int", FluentValue::from(3));
    args.insert("float", FluentValue::from(2.72));

    let value = bundle.format("foo-hit", Some(&args));
    assert_eq!(value, Some("Qux".to_string()));

    let value = bundle.format("foo-miss", Some(&args));
    assert_eq!(value, Some("Foo".to_string()));

    let value = bundle.format("foo-unknown", Some(&args));
    assert_eq!(value, Some("Foo".to_string()));

    let value = bundle.format("bar-hit", Some(&args));
    assert_eq!(value, Some("Bar 3".to_string()));

    let value = bundle.format("bar-miss", Some(&args));
    assert_eq!(value, Some("Bar 1".to_string()));

    let value = bundle.format("bar-unknown", Some(&args));
    assert_eq!(value, Some("Bar 1".to_string()));

    let value = bundle.format("baz-hit", Some(&args));
    assert_eq!(value, Some("Baz E".to_string()));

    let value = bundle.format("baz-miss", Some(&args));
    assert_eq!(value, Some("Baz 1".to_string()));

    let value = bundle.format("baz-unknown", Some(&args));
    assert_eq!(value, Some("Baz 1".to_string()));
}

#[test]
fn select_expression_message_selector() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
-bar = Bar
    .attr = attr val

use-bar =
    { -bar.attr ->
        [attr val] Bar
       *[other] Other
    }
",
    );

    let value = bundle.format("use-bar", None);
    assert_eq!(value, Some("Bar".to_string()));
}

#[test]
fn select_expression_attribute_selector() {
    let mut bundle = FluentBundle::new(&["x-testing"]);
    bundle.add_messages(
        "
-foo = Foo
    .attr = Foo Attr

use-foo =
    { -foo.attr ->
        [Foo Attr] Foo
       *[other] Other
    }
",
    );

    let value = bundle.format("use-foo", None);
    assert_eq!(value, Some("Foo".to_string()));
}