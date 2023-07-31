use prusti_contracts::*;

pub trait GpyType {
    #[pure] // has to be pure if we want to use it in specifications
    fn act<'a>(&self, obj: GpyObject<'a>) -> GpyObject<'a>;
}

pub struct DoublingType;

#[refine_trait_spec]
impl GpyType for DoublingType {
    #[trusted]
    #[pure]
    #[ensures(result.data == obj.data * 2)]
    fn act<'a>(&self, obj: GpyObject<'a>) -> GpyObject<'a> {
        // vv would be replaced by `unreachable!()` because this function should be spec-only
        GpyObject {
            data: obj.data * 2,
            ..obj
        }
    }
}

//#[derive(Clone, Copy)]
pub struct GpyObject {
    pub data: isize,
    pub typ: Box<dyn GpyType>,
}

pub fn pyt_new_object<'a>(data: isize, typ: &'a dyn GpyType) -> GpyObject<'a> {
    GpyObject {
        data,
        typ,
    }
}

#[trusted]
#[ensures(*obj === old(*obj).typ.act(old(*obj)))]
pub fn pyt_apply_action(obj: &mut GpyObject) {
    *obj = obj.typ.act(*obj); // would be replaced by the relevant API call
}
