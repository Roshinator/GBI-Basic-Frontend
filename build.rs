use std::fs;
fn main()
{
    for file in fs::read_dir("ui").unwrap()
    {
        if file.as_ref().unwrap().path().extension().unwrap() == "slint"
        {
            slint_build::compile(file.as_ref().unwrap().path()).unwrap();
        }
    }

}