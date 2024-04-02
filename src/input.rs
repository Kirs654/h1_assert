use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct Opts {

    //公共BBP资产
    /// 获取公共BBP资产
    #[structopt(short="b", long="bbp")]
    pub bbp: bool,

    //私有项目资产
    /// 获取私有项目资产
    #[structopt(short="p", long="private")]
    pub private: bool,


    /// H1 COOKIE信息
    #[structopt(short="c",long="cookie")]
    pub cookie : String,

    ///CSRF token
    #[structopt(short="s",long="csrf")]
    pub csrf_token:String,
    
}

#[warn(unused_mut)]
impl Opts {
    pub fn read() -> Self {
        let opts = Opts::from_args();

        opts
    }

    pub fn validate(&self) {
        if self.bbp && self.private {
            eprintln!("错误：无法同时指定 '-b' 和 '-p' 选项");
            std::process::exit(1);
        }
        if self.cookie.is_empty() || self.csrf_token.is_empty() {
            eprintln!("错误：必须设置 '-c' 和 '-s' 选项");
            std::process::exit(1);
        }
    }

}
