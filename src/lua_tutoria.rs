
fn lua_entry() -> rlua::Result<()> {
    let lua = Lua::new();
    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        globals.set("string_var", "hello")?;
        globals.set("int_var", 42)?;
        Ok(())
    })?;

    #[derive(Copy, Clone, Debug)]
    struct Vec2(f32, f32);

    let mut units: Vec<Vec2> = Vec::new();

    fn add_unit(u: &mut Vec<Vec2>, x: f32, y: f32) {
        u.push(Vec2(x, y));
    }

    add_unit(&mut units, 1.0, 2.0);

    let mut rust_val = 0;

    lua.context(|lua_ctx| {
        lua_ctx
            .scope(|scope| {
                // We create a 'sketchy' Lua callback that holds a mutable reference to the variable
                // `rust_val`.  Outside of a `Context::scope` call, this would not be allowed
                // because it could be unsafe.

                lua_ctx.globals().set(
                    "sketchy",
                    scope.create_function_mut(|_, ()| {
                        rust_val = 42;
                        Ok(())
                    })?,
                )?;

                lua_ctx.load("sketchy()").exec()
            })
            .expect("Failed lua handling");
        println!("rust val: {}", rust_val);
        Ok(())
    })?;

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        println!("{}:", globals.get::<_, String>("string_var")?);
        println!("{}", globals.get::<_, u8>("int_var")?);

        assert_eq!(globals.get::<_, String>("string_var")?, "hello");
        Ok(())
    })?;

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        let p = lua_ctx
            .load(
                r#"
                     global = 'foo'..'bar'
                    "#,
            )
            .exec();
        match p {
            Ok(r) => Ok(r),
            Err(r) => {
                println!("{}", r);
                Err(r)
            }
        }?;

        println!("hello");

        assert_eq!(globals.get::<_, String>("global")?, "foobar");

        let table = lua_ctx.create_table()?;
        table.set(1, "one")?;
        table.set(2, "two")?;
        table.set(3, "three")?;
        assert_eq!(table.len()?, 3);

        globals.set("array_table", table)?;

        lua_ctx
            .load(
                r#"
for k, v in pairs(array_table) do
  print(k, v)
end
"#,
            )
            .exec()?;

        let print: rlua::Function = globals.get("print")?;
        print.call::<_, ()>("hello from rust")?;

        let check_equal = lua_ctx
            .create_function(|_, (list1, list2): (Vec<String>, Vec<String>)| Ok(list1 == list2))?;

        globals.set("check_equal", check_equal)?;

        assert_eq!(
            lua_ctx
                .load(r#"check_equal({"a", "b", "c"}, {"d", "e", "f"})"#)
                .eval::<bool>()?,
            false
        );
        let add_string =
            lua_ctx.create_function(|_, (mut data_v, string_v): (Vec<String>, String)| {
                data_v.push(string_v);
                for i in data_v.iter() {
                    println!("data_v: {}", *i);
                }
                Ok(data_v)
            })?;

        globals.set("add_string", add_string)?;

        let add_string_to =
            lua_ctx.create_function(|_, (obj_name, string_v): (String, String)| {
                Ok((obj_name, string_v))
            })?;

        globals.set("add_string_to", add_string_to)?;

        impl UserData for Vec2 {
            fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
                methods.add_method("magnitude", |_, vec, ()| {
                    let mag_squared = vec.0 * vec.0 + vec.1 * vec.1;
                    Ok(mag_squared.sqrt())
                });
                methods.add_meta_function(MetaMethod::Add, |_, (vec1, vec2): (Vec2, Vec2)| {
                    Ok(Vec2(vec1.0 + vec2.0, vec1.1 + vec2.1))
                });
            }
        }

        let vec2_constructor = lua_ctx.create_function(|_, (x, y): (f32, f32)| Ok(Vec2(x, y)))?;
        globals.set("vec2", vec2_constructor)?;

        lua_ctx
            .load("(vec2(1,2) + vec2(2,2)):magnitude()")
            .eval::<f32>()?;

        let check_equal = lua_ctx
            .create_function(|_, (list1, list2): (Vec<String>, Vec<String>)| Ok(list1 == list2))?;

        globals.set("check_equal", check_equal)?;

        assert_eq!(
            lua_ctx
                .load(r#"check_equal({"a", "b", "c"}, {"d", "e", "f"})"#)
                .eval::<bool>()?,
            false
        );

        print.call::<_, ()>("hello from rust")?;

        Ok(())
    })?;

    lua.context(|lua_ctx| {
        lua_ctx
            .scope(|scope| {
                lua_ctx.globals().set(
                    "add_unit",
                    scope.create_function_mut(|_, (x, y): (f32, f32)| {
                        add_unit(&mut units, x, y);
                        Ok(())
                    })?,
                )?;
                let p = lua_ctx.load("add_unit(1.0, 2.3)").exec();

                let stdin = io::stdin();
                for line in stdin.lock().lines() {
                    let val = line.unwrap();
                    if val == "exit" {
                        break;
                    }
                    let p = lua_ctx.load(&val).exec();
                    match p {
                        Ok(r) => {
                            println!("{:#?}", r);
                        }
                        Err(r) => {
                            println!("{}", r)
                        }
                    }
                }
                p
            })
            .expect("failed lua handling");

        add_unit(&mut units, 2.0, 2.13);

        for i in units.iter() {
            println!("{:#?}", *i);
        }

        Ok(())
    })?;

    Ok(())
}
