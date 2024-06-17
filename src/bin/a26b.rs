// 利用内联模块组织代码 :: 访问模块内部

mod msg {
 pub 暴露出去
}
mod math {

}

fn trim(msg: &str)->&str{
    msg.trim()
}
