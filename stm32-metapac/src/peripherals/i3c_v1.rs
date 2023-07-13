#[doc = "Improved inter-integrated circuit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c {
    ptr: *mut u8,
}
unsafe impl Send for I3c {}
unsafe impl Sync for I3c {}
impl I3c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Control register alternate usage. For the controller to emit a CCC."]
    #[inline(always)]
    pub const fn cr_alt(self) -> crate::common::Reg<regs::CrAlt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "I3C configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "I3C receive data byte register"]
    #[inline(always)]
    pub const fn rdr(self) -> crate::common::Reg<regs::Rdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "I3C receive data word register"]
    #[inline(always)]
    pub const fn rdwr(self) -> crate::common::Reg<regs::Rdwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "I3C transmit data byte register"]
    #[inline(always)]
    pub const fn tdr(self) -> crate::common::Reg<regs::Tdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "I3C transmit data word register"]
    #[inline(always)]
    pub const fn tdwr(self) -> crate::common::Reg<regs::Tdwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "I3C IBI payload data register"]
    #[inline(always)]
    pub const fn ibidr(self) -> crate::common::Reg<regs::Ibidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "I3C target transmit configuration register"]
    #[inline(always)]
    pub const fn tgttdr(self) -> crate::common::Reg<regs::Tgttdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "I3C status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "I3C status error register"]
    #[inline(always)]
    pub const fn ser(self) -> crate::common::Reg<regs::Ser, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "I3C received message register"]
    #[inline(always)]
    pub const fn rmr(self) -> crate::common::Reg<regs::Rmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "I3C event register"]
    #[inline(always)]
    pub const fn evr(self) -> crate::common::Reg<regs::Evr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "I3C interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "I3C clear event register"]
    #[inline(always)]
    pub const fn cevr(self) -> crate::common::Reg<regs::Cevr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "I3C own device characteristics register"]
    #[inline(always)]
    pub const fn devr0(self) -> crate::common::Reg<regs::Devr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "I3C device 1 characteristics register"]
    #[inline(always)]
    pub const fn devr1(self) -> crate::common::Reg<regs::Devr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "I3C device 2 characteristics register"]
    #[inline(always)]
    pub const fn devr2(self) -> crate::common::Reg<regs::Devr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "I3C device 3 characteristics register"]
    #[inline(always)]
    pub const fn devr3(self) -> crate::common::Reg<regs::Devr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "I3C device 4 characteristics register"]
    #[inline(always)]
    pub const fn devr4(self) -> crate::common::Reg<regs::Devr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "I3C maximum read length register"]
    #[inline(always)]
    pub const fn maxrlr(self) -> crate::common::Reg<regs::Maxrlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "I3C maximum write length register"]
    #[inline(always)]
    pub const fn maxwlr(self) -> crate::common::Reg<regs::Maxwlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize) as _) }
    }
    #[doc = "I3C timing register 0"]
    #[inline(always)]
    pub const fn timingr0(self) -> crate::common::Reg<regs::Timingr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "I3C timing register 1"]
    #[inline(always)]
    pub const fn timingr1(self) -> crate::common::Reg<regs::Timingr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "I3C timing register 2"]
    #[inline(always)]
    pub const fn timingr2(self) -> crate::common::Reg<regs::Timingr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(168usize) as _) }
    }
    #[doc = "I3C bus characteristics register"]
    #[inline(always)]
    pub const fn bcr(self) -> crate::common::Reg<regs::Bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "I3C device characteristics register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(196usize) as _) }
    }
    #[doc = "I3C get capability register"]
    #[inline(always)]
    pub const fn getcapr(self) -> crate::common::Reg<regs::Getcapr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(200usize) as _) }
    }
    #[doc = "I3C controller-role capability register"]
    #[inline(always)]
    pub const fn crcapr(self) -> crate::common::Reg<regs::Crcapr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(204usize) as _) }
    }
    #[doc = "I3C get capability register"]
    #[inline(always)]
    pub const fn getmxdsr(self) -> crate::common::Reg<regs::Getmxdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(208usize) as _) }
    }
    #[doc = "I3C extended provisioned ID register"]
    #[inline(always)]
    pub const fn epidr(self) -> crate::common::Reg<regs::Epidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(212usize) as _) }
    }
}
pub mod regs {
    #[doc = "I3C bus characteristics register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr(pub u32);
    impl Bcr {
        #[doc = "max data speed limitation. 0 no limitation. 1 limitation, as described by GETMXDSR.\""]
        #[inline(always)]
        pub const fn bcr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "max data speed limitation. 0 no limitation. 1 limitation, as described by GETMXDSR.\""]
        #[inline(always)]
        pub fn set_bcr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "in-band interrupt (IBI) payload. 1 = at least one mandatory data byte follows the accepted IBI (and at most 4 data bytes)"]
        #[inline(always)]
        pub const fn bcr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "in-band interrupt (IBI) payload. 1 = at least one mandatory data byte follows the accepted IBI (and at most 4 data bytes)"]
        #[inline(always)]
        pub fn set_bcr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "controller capable. 1 = capable."]
        #[inline(always)]
        pub const fn bcr6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "controller capable. 1 = capable."]
        #[inline(always)]
        pub fn set_bcr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Bcr {
        #[inline(always)]
        fn default() -> Bcr {
            Bcr(0)
        }
    }
    #[doc = "I3C clear event register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cevr(pub u32);
    impl Cevr {
        #[doc = "clear frame complete flag (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub const fn cfcf(&self) -> super::vals::Cfcf {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Cfcf::from_bits(val as u8)
        }
        #[doc = "clear frame complete flag (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub fn set_cfcf(&mut self, val: super::vals::Cfcf) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "clear target-initiated read end flag (when the I3C is acting as controller)"]
        #[inline(always)]
        pub const fn crxtgtendf(&self) -> super::vals::Crxtgtendf {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Crxtgtendf::from_bits(val as u8)
        }
        #[doc = "clear target-initiated read end flag (when the I3C is acting as controller)"]
        #[inline(always)]
        pub fn set_crxtgtendf(&mut self, val: super::vals::Crxtgtendf) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "clear error flag (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub const fn cerrf(&self) -> super::vals::Cerrf {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Cerrf::from_bits(val as u8)
        }
        #[doc = "clear error flag (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub fn set_cerrf(&mut self, val: super::vals::Cerrf) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "clear IBI request flag (when the I3C is acting as controller)"]
        #[inline(always)]
        pub const fn cibif(&self) -> super::vals::Cibif {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Cibif::from_bits(val as u8)
        }
        #[doc = "clear IBI request flag (when the I3C is acting as controller)"]
        #[inline(always)]
        pub fn set_cibif(&mut self, val: super::vals::Cibif) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "clear IBI end flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn cibiendf(&self) -> super::vals::Cibiendf {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Cibiendf::from_bits(val as u8)
        }
        #[doc = "clear IBI end flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_cibiendf(&mut self, val: super::vals::Cibiendf) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "clear controller-role request flag (when the I3C is acting as controller)"]
        #[inline(always)]
        pub const fn ccrf(&self) -> super::vals::Ccrf {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Ccrf::from_bits(val as u8)
        }
        #[doc = "clear controller-role request flag (when the I3C is acting as controller)"]
        #[inline(always)]
        pub fn set_ccrf(&mut self, val: super::vals::Ccrf) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "clear controller-role update flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn ccrupdf(&self) -> super::vals::Ccrupdf {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Ccrupdf::from_bits(val as u8)
        }
        #[doc = "clear controller-role update flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_ccrupdf(&mut self, val: super::vals::Ccrupdf) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "clear hot-join flag (when the I3C is acting as controller)"]
        #[inline(always)]
        pub const fn chjf(&self) -> super::vals::Chjf {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Chjf::from_bits(val as u8)
        }
        #[doc = "clear hot-join flag (when the I3C is acting as controller)"]
        #[inline(always)]
        pub fn set_chjf(&mut self, val: super::vals::Chjf) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "clear wakeup flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn cwkpf(&self) -> super::vals::Cwkpf {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Cwkpf::from_bits(val as u8)
        }
        #[doc = "clear wakeup flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_cwkpf(&mut self, val: super::vals::Cwkpf) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "clear GETxxx CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn cgetf(&self) -> super::vals::Cgetf {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Cgetf::from_bits(val as u8)
        }
        #[doc = "clear GETxxx CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_cgetf(&mut self, val: super::vals::Cgetf) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "clear GETSTATUS CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn cstaf(&self) -> super::vals::Cstaf {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Cstaf::from_bits(val as u8)
        }
        #[doc = "clear GETSTATUS CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_cstaf(&mut self, val: super::vals::Cstaf) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn cdaupdf(&self) -> super::vals::Cdaupdf {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Cdaupdf::from_bits(val as u8)
        }
        #[doc = "clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_cdaupdf(&mut self, val: super::vals::Cdaupdf) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "clear SETMWL CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn cmwlupdf(&self) -> super::vals::Cmwlupdf {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Cmwlupdf::from_bits(val as u8)
        }
        #[doc = "clear SETMWL CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_cmwlupdf(&mut self, val: super::vals::Cmwlupdf) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "clear SETMRL CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn cmrlupdf(&self) -> super::vals::Cmrlupdf {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Cmrlupdf::from_bits(val as u8)
        }
        #[doc = "clear SETMRL CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_cmrlupdf(&mut self, val: super::vals::Cmrlupdf) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
        #[doc = "clear reset pattern flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn crstf(&self) -> super::vals::Crstf {
            let val = (self.0 >> 27usize) & 0x01;
            super::vals::Crstf::from_bits(val as u8)
        }
        #[doc = "clear reset pattern flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_crstf(&mut self, val: super::vals::Crstf) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
        }
        #[doc = "clear ENTASx CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn casupdf(&self) -> super::vals::Casupdf {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Casupdf::from_bits(val as u8)
        }
        #[doc = "clear ENTASx CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_casupdf(&mut self, val: super::vals::Casupdf) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "clear ENEC/DISEC CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn cintupdf(&self) -> super::vals::Cintupdf {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Cintupdf::from_bits(val as u8)
        }
        #[doc = "clear ENEC/DISEC CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_cintupdf(&mut self, val: super::vals::Cintupdf) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "clear DEFTGTS CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn cdeff(&self) -> super::vals::Cdeff {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Cdeff::from_bits(val as u8)
        }
        #[doc = "clear DEFTGTS CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_cdeff(&mut self, val: super::vals::Cdeff) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "clear DEFGRPA CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn cgrpf(&self) -> super::vals::Cgrpf {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Cgrpf::from_bits(val as u8)
        }
        #[doc = "clear DEFGRPA CCC flag (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_cgrpf(&mut self, val: super::vals::Cgrpf) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cevr {
        #[inline(always)]
        fn default() -> Cevr {
            Cevr(0)
        }
    }
    #[doc = "I3C configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the CFGR)"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the CFGR)"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "initial controller/target role This bit can be modified only when CFGR.EN = 0. Once enabled by setting CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
        #[inline(always)]
        pub const fn crinit(&self) -> super::vals::Crinit {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Crinit::from_bits(val as u8)
        }
        #[doc = "initial controller/target role This bit can be modified only when CFGR.EN = 0. Once enabled by setting CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
        #[inline(always)]
        pub fn set_crinit(&mut self, val: super::vals::Crinit) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR)."]
        #[inline(always)]
        pub const fn noarbh(&self) -> super::vals::Noarbh {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Noarbh::from_bits(val as u8)
        }
        #[doc = "no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR)."]
        #[inline(always)]
        pub fn set_noarbh(&mut self, val: super::vals::Noarbh) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame."]
        #[inline(always)]
        pub const fn rstptrn(&self) -> super::vals::Rstptrn {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Rstptrn::from_bits(val as u8)
        }
        #[doc = "HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame."]
        #[inline(always)]
        pub fn set_rstptrn(&mut self, val: super::vals::Rstptrn) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn’t assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed."]
        #[inline(always)]
        pub const fn exitptrn(&self) -> super::vals::Exitptrn {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Exitptrn::from_bits(val as u8)
        }
        #[doc = "HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn’t assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed."]
        #[inline(always)]
        pub fn set_exitptrn(&mut self, val: super::vals::Exitptrn) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when CFGR.EN=0."]
        #[inline(always)]
        pub const fn hksdaen(&self) -> super::vals::Hksdaen {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Hksdaen::from_bits(val as u8)
        }
        #[doc = "High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when CFGR.EN=0."]
        #[inline(always)]
        pub fn set_hksdaen(&mut self, val: super::vals::Hksdaen) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again."]
        #[inline(always)]
        pub const fn hjack(&self) -> super::vals::Hjack {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Hjack::from_bits(val as u8)
        }
        #[doc = "Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again."]
        #[inline(always)]
        pub fn set_hjack(&mut self, val: super::vals::Hjack) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads RDR or RDWR register. - A next data byte/word is to be read by the software either via polling on the flag EVR.RXFNEF=1 or via interrupt notification (enabled by IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads RDR or RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> super::vals::Rxdmaen {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Rxdmaen::from_bits(val as u8)
        }
        #[doc = "RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads RDR or RDWR register. - A next data byte/word is to be read by the software either via polling on the flag EVR.RXFNEF=1 or via interrupt notification (enabled by IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads RDR or RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub fn set_rxdmaen(&mut self, val: super::vals::Rxdmaen) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "RX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written."]
        #[inline(always)]
        pub const fn rxflush(&self) -> super::vals::Rxflush {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Rxflush::from_bits(val as u8)
        }
        #[doc = "RX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written."]
        #[inline(always)]
        pub fn set_rxflush(&mut self, val: super::vals::Rxflush) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in RDWR)."]
        #[inline(always)]
        pub const fn rxthres(&self) -> super::vals::Rxthres {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Rxthres::from_bits(val as u8)
        }
        #[doc = "RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in RDWR)."]
        #[inline(always)]
        pub fn set_rxthres(&mut self, val: super::vals::Rxthres) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes TDR or TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag EVR.TXFNFF=1 or via interrupt notification (enabled by IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes TDR or TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub const fn txdmaen(&self) -> super::vals::Txdmaen {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Txdmaen::from_bits(val as u8)
        }
        #[doc = "TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes TDR or TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag EVR.TXFNFF=1 or via interrupt notification (enabled by IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes TDR or TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub fn set_txdmaen(&mut self, val: super::vals::Txdmaen) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "TX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written. When the I3C is acting as target, this bit can be used to flush the TX-FIFO on a private read if the controller has early ended the read data (i.e. driven low the T bit) and there is/are remaining data in the TX-FIFO (i.e. SR.ABT=1 and SR.XDCNT\\[15:0\\]
< TGTTDR.TGTTDCNT\\[15:0\\])."]
        #[inline(always)]
        pub const fn txflush(&self) -> super::vals::Txflush {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Txflush::from_bits(val as u8)
        }
        #[doc = "TX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written. When the I3C is acting as target, this bit can be used to flush the TX-FIFO on a private read if the controller has early ended the read data (i.e. driven low the T bit) and there is/are remaining data in the TX-FIFO (i.e. SR.ABT=1 and SR.XDCNT\\[15:0\\]
< TGTTDR.TGTTDCNT\\[15:0\\])."]
        #[inline(always)]
        pub fn set_txflush(&mut self, val: super::vals::Txflush) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in TDWR)."]
        #[inline(always)]
        pub const fn txthres(&self) -> super::vals::Txthres {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Txthres::from_bits(val as u8)
        }
        #[doc = "TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in TDWR)."]
        #[inline(always)]
        pub fn set_txthres(&mut self, val: super::vals::Txthres) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads SR register after a completed frame (EVR.FCF=1) or an error (EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by IER.FCIE=1 and IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub const fn sdmaen(&self) -> super::vals::Sdmaen {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Sdmaen::from_bits(val as u8)
        }
        #[doc = "S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads SR register after a completed frame (EVR.FCF=1) or an error (EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by IER.FCIE=1 and IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub fn set_sdmaen(&mut self, val: super::vals::Sdmaen) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "S-FIFO flush (when I3C is acting as controller) When I3C is acting as I3C controller, this bit can only be written (and is only used when I3C is acting as controller)."]
        #[inline(always)]
        pub const fn sflush(&self) -> super::vals::Sflush {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Sflush::from_bits(val as u8)
        }
        #[doc = "S-FIFO flush (when I3C is acting as controller) When I3C is acting as I3C controller, this bit can only be written (and is only used when I3C is acting as controller)."]
        #[inline(always)]
        pub fn set_sflush(&mut self, val: super::vals::Sflush) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO."]
        #[inline(always)]
        pub const fn rmode(&self) -> super::vals::Rmode {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Rmode::from_bits(val as u8)
        }
        #[doc = "S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO."]
        #[inline(always)]
        pub fn set_rmode(&mut self, val: super::vals::Rmode) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO."]
        #[inline(always)]
        pub const fn tmode(&self) -> super::vals::Tmode {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Tmode::from_bits(val as u8)
        }
        #[doc = "transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO."]
        #[inline(always)]
        pub fn set_tmode(&mut self, val: super::vals::Tmode) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag EVR.CFNFF=1 or via interrupt notification (enabled by IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub const fn cdmaen(&self) -> super::vals::Cdmaen {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Cdmaen::from_bits(val as u8)
        }
        #[doc = "C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag EVR.CFNFF=1 or via interrupt notification (enabled by IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel)."]
        #[inline(always)]
        pub fn set_cdmaen(&mut self, val: super::vals::Cdmaen) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "C-FIFO flush (when I3C is acting as controller) This bit can only be written."]
        #[inline(always)]
        pub const fn cflush(&self) -> super::vals::Cflush {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Cflush::from_bits(val as u8)
        }
        #[doc = "C-FIFO flush (when I3C is acting as controller) This bit can only be written."]
        #[inline(always)]
        pub fn set_cflush(&mut self, val: super::vals::Cflush) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "frame transfer set (a.k.a. software trigger) (when I3C is acting as controller) This bit can only be written. When I3C is acting as I3C controller: Note: If this bit is not set, the other alternative for the software to initiate a frame transfer is to directly write the first control word register (i.e. CR) while C-FIFO is empty (i.e. EVR.CFEF=1). Then, if the first written control word is not tagged as a message end (i.e CR.MEND=0), it causes the hardware to assert the flag EVR.CFNFF (C-FIFO not full and a next control word is needed)."]
        #[inline(always)]
        pub const fn tsfset(&self) -> super::vals::Tsfset {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Tsfset::from_bits(val as u8)
        }
        #[doc = "frame transfer set (a.k.a. software trigger) (when I3C is acting as controller) This bit can only be written. When I3C is acting as I3C controller: Note: If this bit is not set, the other alternative for the software to initiate a frame transfer is to directly write the first control word register (i.e. CR) while C-FIFO is empty (i.e. EVR.CFEF=1). Then, if the first written control word is not tagged as a message end (i.e CR.MEND=0), it causes the hardware to assert the flag EVR.CFNFF (C-FIFO not full and a next control word is needed)."]
        #[inline(always)]
        pub fn set_tsfset(&mut self, val: super::vals::Tsfset) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "I3C message control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "count of data to transfer during a read or write message, in bytes (whatever I3C is acting as controller/target) Linear encoding up to 64 Kbytes -1 ..."]
        #[inline(always)]
        pub const fn dcnt(&self) -> super::vals::Dcnt {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Dcnt::from_bits(val as u16)
        }
        #[doc = "count of data to transfer during a read or write message, in bytes (whatever I3C is acting as controller/target) Linear encoding up to 64 Kbytes -1 ..."]
        #[inline(always)]
        pub fn set_dcnt(&mut self, val: super::vals::Dcnt) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
        }
        #[doc = "read / non-write message (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message), in order to emit the RnW bit on the I3C bus."]
        #[inline(always)]
        pub const fn rnw(&self) -> super::vals::Rnw {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Rnw::from_bits(val as u8)
        }
        #[doc = "read / non-write message (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message), in order to emit the RnW bit on the I3C bus."]
        #[inline(always)]
        pub fn set_rnw(&mut self, val: super::vals::Rnw) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "7-bit I3C dynamic / I2C static target address (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message)"]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "7-bit I3C dynamic / I2C static target address (when I3C is acting as controller) When I3C is acting as controller, this field is used if MTYPE\\[3:0\\]=0010 (private message) or MTYPE\\[3:0\\]=0011 (direct message) or MTYPE\\[3:0\\]=0100 (legacy I2C message)"]
        #[inline(always)]
        pub fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
        #[doc = "message type (whatever I3C is acting as controller/target) Bits\\[26:0\\]
are ignored. After M2 error detection on an I3C SDR message, this is needed for SCL “stuck at” recovery. Bits\\[26:0\\]
are ignored. If CFGR.EXITPTRN=1, an HDR exit pattern is emitted on the bus to generate an escalation fault. Bits\\[23:17\\]
(ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\]
(RNW) is the emitted RnW bit. The transferred private message is: {S / S+7’h7E+RnW=0+Sr / Sr+*} + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). Bits\\[23:17\\]
(ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\]
(RNW) is the emitted RnW bit. The transferred direct message is: Sr + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P Bits\\[23:17\\]
(ADD\\[6:0\\]) is the emitted 7-bit static address. Bit\\[16\\]
(RNW) is the emitted RnW bit. The transferred legacy I2C message is: {S / S+ 7’h7E+RnW=0 + Sr / Sr+*} + 7-bit StaAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). 1xxx: reserved (when I3C is acting as I3C controller, used when target) 0xxx: reserved {S +} 7’h02 addr + RnW=0 {S +} 7-bit DEVR0.DA\\[6:0\\]
+ RnW=0 after a bus available condition (the target first emits a START request), or once the controller drives a START. {S +} 7-bit DEVR0.DA\\[6:0\\]
+ RnW=1 (+Ack/Nack from controller) When acknowledged from controller, the next (optional, depending on BCR.BCR2) transmitted IBI payload data is defined by CR.DCNT\\[15:0\\]
and must be consistently programmed vs the maximum IBI payload data size which is defined by IBIDR.IBIP\\[2:0\\]. Others: reserved"]
        #[inline(always)]
        pub const fn mtype(&self) -> super::vals::Mtype {
            let val = (self.0 >> 27usize) & 0x0f;
            super::vals::Mtype::from_bits(val as u8)
        }
        #[doc = "message type (whatever I3C is acting as controller/target) Bits\\[26:0\\]
are ignored. After M2 error detection on an I3C SDR message, this is needed for SCL “stuck at” recovery. Bits\\[26:0\\]
are ignored. If CFGR.EXITPTRN=1, an HDR exit pattern is emitted on the bus to generate an escalation fault. Bits\\[23:17\\]
(ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\]
(RNW) is the emitted RnW bit. The transferred private message is: {S / S+7’h7E+RnW=0+Sr / Sr+*} + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). Bits\\[23:17\\]
(ADD\\[6:0\\]) is the emitted 7-bit dynamic address. Bit\\[16\\]
(RNW) is the emitted RnW bit. The transferred direct message is: Sr + 7-bit DynAddr + RnW + (8-bit Data + T)* + Sr/P Bits\\[23:17\\]
(ADD\\[6:0\\]) is the emitted 7-bit static address. Bit\\[16\\]
(RNW) is the emitted RnW bit. The transferred legacy I2C message is: {S / S+ 7’h7E+RnW=0 + Sr / Sr+*} + 7-bit StaAddr + RnW + (8-bit Data + T)* + Sr/P. After a S (START), depending on CFGR.NOARBH, the arbitrable header (7’h7E+RnW=0) is inserted or not. Sr+*: after a Sr (Repeated Start), the hardware automatically inserts (7’h7E+RnW=0) if needed, i.e. if it follows an I3C direct message without ending by a P (Stop). 1xxx: reserved (when I3C is acting as I3C controller, used when target) 0xxx: reserved {S +} 7’h02 addr + RnW=0 {S +} 7-bit DEVR0.DA\\[6:0\\]
+ RnW=0 after a bus available condition (the target first emits a START request), or once the controller drives a START. {S +} 7-bit DEVR0.DA\\[6:0\\]
+ RnW=1 (+Ack/Nack from controller) When acknowledged from controller, the next (optional, depending on BCR.BCR2) transmitted IBI payload data is defined by CR.DCNT\\[15:0\\]
and must be consistently programmed vs the maximum IBI payload data size which is defined by IBIDR.IBIP\\[2:0\\]. Others: reserved"]
        #[inline(always)]
        pub fn set_mtype(&mut self, val: super::vals::Mtype) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val.to_bits() as u32) & 0x0f) << 27usize);
        }
        #[doc = "message end type (when the I3C is acting as controller)"]
        #[inline(always)]
        pub const fn mend(&self) -> super::vals::Mend {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Mend::from_bits(val as u8)
        }
        #[doc = "message end type (when the I3C is acting as controller)"]
        #[inline(always)]
        pub fn set_mend(&mut self, val: super::vals::Mend) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "Control register alternate usage"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CrAlt(pub u32);
    impl CrAlt {
        #[doc = "count of data to transfer during a read or write message, in bytes (when I3C is acting as controller) Linear encoding up to 64 Kbytes -1. ..."]
        #[inline(always)]
        pub const fn dcnt(&self) -> super::vals::Dcnt {
            let val = (self.0 >> 0usize) & 0xffff;
            super::vals::Dcnt::from_bits(val as u16)
        }
        #[doc = "count of data to transfer during a read or write message, in bytes (when I3C is acting as controller) Linear encoding up to 64 Kbytes -1. ..."]
        #[inline(always)]
        pub fn set_dcnt(&mut self, val: super::vals::Dcnt) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
        }
        #[doc = "8-bit CCC code (when I3C is acting as controller) If Bit\\[23\\]=CCC\\[7\\]=1, this is the 1st part of an I3C SDR direct CCC command. If Bit\\[23\\]=CCC\\[7\\]=0, this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0)."]
        #[inline(always)]
        pub const fn ccc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit CCC code (when I3C is acting as controller) If Bit\\[23\\]=CCC\\[7\\]=1, this is the 1st part of an I3C SDR direct CCC command. If Bit\\[23\\]=CCC\\[7\\]=0, this is an I3C SDR broadcast CCC command (including ENTDAA and ENTHDR0)."]
        #[inline(always)]
        pub fn set_ccc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "MTYPE must be 0110 for alternate usage"]
        #[inline(always)]
        pub const fn mtype(&self) -> super::vals::Mtype {
            let val = (self.0 >> 27usize) & 0x0f;
            super::vals::Mtype::from_bits(val as u8)
        }
        #[doc = "MTYPE must be 0110 for alternate usage"]
        #[inline(always)]
        pub fn set_mtype(&mut self, val: super::vals::Mtype) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val.to_bits() as u32) & 0x0f) << 27usize);
        }
        #[doc = "message end type (when I3C is acting as controller)"]
        #[inline(always)]
        pub const fn mend(&self) -> super::vals::Mend {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Mend::from_bits(val as u8)
        }
        #[doc = "message end type (when I3C is acting as controller)"]
        #[inline(always)]
        pub fn set_mend(&mut self, val: super::vals::Mend) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CrAlt {
        #[inline(always)]
        fn default() -> CrAlt {
            CrAlt(0)
        }
    }
    #[doc = "I3C controller-role capability register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcapr(pub u32);
    impl Crcapr {
        #[doc = "delayed controller-role hand-off This bit is written by software during bus initialization (i.e. CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2."]
        #[inline(always)]
        pub const fn capdhoff(&self) -> super::vals::Capdhoff {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Capdhoff::from_bits(val as u8)
        }
        #[doc = "delayed controller-role hand-off This bit is written by software during bus initialization (i.e. CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2."]
        #[inline(always)]
        pub fn set_capdhoff(&mut self, val: super::vals::Capdhoff) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "group management support (when acting as controller) This bit is written by software during bus initialization (i.e. CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2."]
        #[inline(always)]
        pub const fn capgrp(&self) -> super::vals::Capgrp {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Capgrp::from_bits(val as u8)
        }
        #[doc = "group management support (when acting as controller) This bit is written by software during bus initialization (i.e. CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2."]
        #[inline(always)]
        pub fn set_capgrp(&mut self, val: super::vals::Capgrp) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Crcapr {
        #[inline(always)]
        fn default() -> Crcapr {
            Crcapr(0)
        }
    }
    #[doc = "I3C device characteristics register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr(pub u32);
    impl Dcr {
        #[doc = "device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_device_characteristics_register"]
        #[inline(always)]
        pub const fn dcr(&self) -> super::vals::Dcr {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Dcr::from_bits(val as u8)
        }
        #[doc = "device characteristics ID others: ID to describe the type of the I3C sensor/device Note: The latest MIPI DCR ID assignments are available at: https://www.mipi.org/MIPI_device_characteristics_register"]
        #[inline(always)]
        pub fn set_dcr(&mut self, val: super::vals::Dcr) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dcr {
        #[inline(always)]
        fn default() -> Dcr {
            Dcr(0)
        }
    }
    #[doc = "I3C own device characteristics register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devr0(pub u32);
    impl Devr0 {
        #[doc = "dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
        #[inline(always)]
        pub const fn daval(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
        #[inline(always)]
        pub fn set_daval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
        #[inline(always)]
        pub fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "IBI request enable (when the I3C is acting as target) This field is initially written by software when CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
        #[inline(always)]
        pub const fn ibien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IBI request enable (when the I3C is acting as target) This field is initially written by software when CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
        #[inline(always)]
        pub fn set_ibien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "controller-role request enable (when the I3C is acting as target) This field is initially written by software when CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
        #[inline(always)]
        pub const fn cren(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "controller-role request enable (when the I3C is acting as target) This field is initially written by software when CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
        #[inline(always)]
        pub fn set_cren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "hot-join request enable (when the I3C is acting as target) This field is initially written by software when CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
        #[inline(always)]
        pub const fn hjen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "hot-join request enable (when the I3C is acting as target) This field is initially written by software when CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
        #[inline(always)]
        pub fn set_hjen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):"]
        #[inline(always)]
        pub const fn as_(&self) -> super::vals::As {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::As::from_bits(val as u8)
        }
        #[doc = "activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):"]
        #[inline(always)]
        pub fn set_as_(&mut self, val: super::vals::As) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus. This read field is updated by hardware on the reception of a broadcast or direct write RSTACT CCC (target reset action). Only the defining bytes 0x00, 0x01 and 0x02 are mapped, and RSTACT\\[1:0\\]
= Defining Byte\\[1:0\\]. a) partially reset the I3C peripheral, by a write and clear of the enable bit of the i3C configuration register (i.e. write CFGR.EN=0). This reset the I3C bus interface and the I3C kernel sub-parts, without modifying the content of the I3C APB registers (excepted the CFGR.EN bit). b) reset fully the I3C peripheral including all its registers via a write and set to the I3C reset control bit of the RCC (Reset and Clock Controller) register. a system reset. This has the same impact as a pin reset (i.e. NRST=0) (refer to RCC functional description - Reset part): – the software writes and set the AICR.SYSRESETREQ register control bit, when the device is controlled by a CortexTM-M. – the software writes and set the RCC_GRSTCSETR.SYSRST=1, when the device is controlled by a CortexTM-A."]
        #[inline(always)]
        pub const fn rstact(&self) -> super::vals::Rstact {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Rstact::from_bits(val as u8)
        }
        #[doc = "reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus. This read field is updated by hardware on the reception of a broadcast or direct write RSTACT CCC (target reset action). Only the defining bytes 0x00, 0x01 and 0x02 are mapped, and RSTACT\\[1:0\\]
= Defining Byte\\[1:0\\]. a) partially reset the I3C peripheral, by a write and clear of the enable bit of the i3C configuration register (i.e. write CFGR.EN=0). This reset the I3C bus interface and the I3C kernel sub-parts, without modifying the content of the I3C APB registers (excepted the CFGR.EN bit). b) reset fully the I3C peripheral including all its registers via a write and set to the I3C reset control bit of the RCC (Reset and Clock Controller) register. a system reset. This has the same impact as a pin reset (i.e. NRST=0) (refer to RCC functional description - Reset part): – the software writes and set the AICR.SYSRESETREQ register control bit, when the device is controlled by a CortexTM-M. – the software writes and set the RCC_GRSTCSETR.SYSRST=1, when the device is controlled by a CortexTM-A."]
        #[inline(always)]
        pub fn set_rstact(&mut self, val: super::vals::Rstact) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\\[1:0\\]
field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), DEVR0.RSTACT\\[1:0\\]
dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one."]
        #[inline(always)]
        pub const fn rstval(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\\[1:0\\]
field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), DEVR0.RSTACT\\[1:0\\]
dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one."]
        #[inline(always)]
        pub fn set_rstval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Devr0 {
        #[inline(always)]
        fn default() -> Devr0 {
            Devr0(0)
        }
    }
    #[doc = "I3C device 1 characteristics register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devr1(pub u32);
    impl Devr1 {
        #[doc = "assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. EVR.IBIF) and controller-role request flag (i.e. EVR.CRF) are both cleared."]
        #[inline(always)]
        pub const fn ibiack(&self) -> super::vals::Devr1Ibiack {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Devr1Ibiack::from_bits(val as u8)
        }
        #[doc = "IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. EVR.IBIF) and controller-role request flag (i.e. EVR.CRF) are both cleared."]
        #[inline(always)]
        pub fn set_ibiack(&mut self, val: super::vals::Devr1Ibiack) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. EVR.CRF) and IBI flag (i.e. EVR.IBIF) are both cleared."]
        #[inline(always)]
        pub const fn crack(&self) -> super::vals::Devr1Crack {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Devr1Crack::from_bits(val as u8)
        }
        #[doc = "controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. EVR.CRF) and IBI flag (i.e. EVR.IBIF) are both cleared."]
        #[inline(always)]
        pub fn set_crack(&mut self, val: super::vals::Devr1Crack) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\]
bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received RDR. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub const fn ibiden(&self) -> super::vals::Devr1Ibiden {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Devr1Ibiden::from_bits(val as u8)
        }
        #[doc = "IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\]
bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received RDR. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub fn set_ibiden(&mut self, val: super::vals::Devr1Ibiden) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3’b101). If this bit is set, when an IBI is received (i.e. EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
        #[inline(always)]
        pub const fn susp(&self) -> super::vals::Devr1Susp {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Devr1Susp::from_bits(val as u8)
        }
        #[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3’b101). If this bit is set, when an IBI is received (i.e. EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
        #[inline(always)]
        pub fn set_susp(&mut self, val: super::vals::Devr1Susp) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "DA\\[6:0\\]
write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\]
and IBIDEN values. Then, to be able to next modify DA\\[6:0\\]
or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\]
or IBIDEN."]
        #[inline(always)]
        pub const fn dis(&self) -> super::vals::Devr1Dis {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Devr1Dis::from_bits(val as u8)
        }
        #[doc = "DA\\[6:0\\]
write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\]
and IBIDEN values. Then, to be able to next modify DA\\[6:0\\]
or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\]
or IBIDEN."]
        #[inline(always)]
        pub fn set_dis(&mut self, val: super::vals::Devr1Dis) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Devr1 {
        #[inline(always)]
        fn default() -> Devr1 {
            Devr1(0)
        }
    }
    #[doc = "I3C device 2 characteristics register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devr2(pub u32);
    impl Devr2 {
        #[doc = "assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. EVR.IBIF) and controller-role request flag (i.e. EVR.CRF) are both cleared."]
        #[inline(always)]
        pub const fn ibiack(&self) -> super::vals::Devr2Ibiack {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Devr2Ibiack::from_bits(val as u8)
        }
        #[doc = "IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. EVR.IBIF) and controller-role request flag (i.e. EVR.CRF) are both cleared."]
        #[inline(always)]
        pub fn set_ibiack(&mut self, val: super::vals::Devr2Ibiack) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. EVR.CRF) and IBI flag (i.e. EVR.IBIF) are both cleared."]
        #[inline(always)]
        pub const fn crack(&self) -> super::vals::Devr2Crack {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Devr2Crack::from_bits(val as u8)
        }
        #[doc = "controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. EVR.CRF) and IBI flag (i.e. EVR.IBIF) are both cleared."]
        #[inline(always)]
        pub fn set_crack(&mut self, val: super::vals::Devr2Crack) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\]
bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received RDR. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub const fn ibiden(&self) -> super::vals::Devr2Ibiden {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Devr2Ibiden::from_bits(val as u8)
        }
        #[doc = "IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\]
bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received RDR. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub fn set_ibiden(&mut self, val: super::vals::Devr2Ibiden) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3’b101). If this bit is set, when an IBI is received (i.e. EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
        #[inline(always)]
        pub const fn susp(&self) -> super::vals::Devr2Susp {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Devr2Susp::from_bits(val as u8)
        }
        #[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3’b101). If this bit is set, when an IBI is received (i.e. EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
        #[inline(always)]
        pub fn set_susp(&mut self, val: super::vals::Devr2Susp) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "DA\\[6:0\\]
write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\]
and IBIDEN values. Then, to be able to next modify DA\\[6:0\\]
or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\]
or IBIDEN."]
        #[inline(always)]
        pub const fn dis(&self) -> super::vals::Devr2Dis {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Devr2Dis::from_bits(val as u8)
        }
        #[doc = "DA\\[6:0\\]
write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\]
and IBIDEN values. Then, to be able to next modify DA\\[6:0\\]
or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\]
or IBIDEN."]
        #[inline(always)]
        pub fn set_dis(&mut self, val: super::vals::Devr2Dis) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Devr2 {
        #[inline(always)]
        fn default() -> Devr2 {
            Devr2(0)
        }
    }
    #[doc = "I3C device 3 characteristics register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devr3(pub u32);
    impl Devr3 {
        #[doc = "assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. EVR.IBIF) and controller-role request flag (i.e. EVR.CRF) are both cleared."]
        #[inline(always)]
        pub const fn ibiack(&self) -> super::vals::Devr3Ibiack {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Devr3Ibiack::from_bits(val as u8)
        }
        #[doc = "IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. EVR.IBIF) and controller-role request flag (i.e. EVR.CRF) are both cleared."]
        #[inline(always)]
        pub fn set_ibiack(&mut self, val: super::vals::Devr3Ibiack) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. EVR.CRF) and IBI flag (i.e. EVR.IBIF) are both cleared."]
        #[inline(always)]
        pub const fn crack(&self) -> super::vals::Devr3Crack {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Devr3Crack::from_bits(val as u8)
        }
        #[doc = "controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. EVR.CRF) and IBI flag (i.e. EVR.IBIF) are both cleared."]
        #[inline(always)]
        pub fn set_crack(&mut self, val: super::vals::Devr3Crack) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\]
bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received RDR. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub const fn ibiden(&self) -> super::vals::Devr3Ibiden {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Devr3Ibiden::from_bits(val as u8)
        }
        #[doc = "IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\]
bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received RDR. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub fn set_ibiden(&mut self, val: super::vals::Devr3Ibiden) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3’b101). If this bit is set, when an IBI is received (i.e. EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
        #[inline(always)]
        pub const fn susp(&self) -> super::vals::Devr3Susp {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Devr3Susp::from_bits(val as u8)
        }
        #[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3’b101). If this bit is set, when an IBI is received (i.e. EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
        #[inline(always)]
        pub fn set_susp(&mut self, val: super::vals::Devr3Susp) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "DA\\[6:0\\]
write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\]
and IBIDEN values. Then, to be able to next modify DA\\[6:0\\]
or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\]
or IBIDEN."]
        #[inline(always)]
        pub const fn dis(&self) -> super::vals::Devr3Dis {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Devr3Dis::from_bits(val as u8)
        }
        #[doc = "DA\\[6:0\\]
write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\]
and IBIDEN values. Then, to be able to next modify DA\\[6:0\\]
or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\]
or IBIDEN."]
        #[inline(always)]
        pub fn set_dis(&mut self, val: super::vals::Devr3Dis) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Devr3 {
        #[inline(always)]
        fn default() -> Devr3 {
            Devr3(0)
        }
    }
    #[doc = "I3C device 4 characteristics register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devr4(pub u32);
    impl Devr4 {
        #[doc = "assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. EVR.IBIF) and controller-role request flag (i.e. EVR.CRF) are both cleared."]
        #[inline(always)]
        pub const fn ibiack(&self) -> super::vals::Devr4Ibiack {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Devr4Ibiack::from_bits(val as u8)
        }
        #[doc = "IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. EVR.IBIF) and controller-role request flag (i.e. EVR.CRF) are both cleared."]
        #[inline(always)]
        pub fn set_ibiack(&mut self, val: super::vals::Devr4Ibiack) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. EVR.CRF) and IBI flag (i.e. EVR.IBIF) are both cleared."]
        #[inline(always)]
        pub const fn crack(&self) -> super::vals::Devr4Crack {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Devr4Crack::from_bits(val as u8)
        }
        #[doc = "controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\\[6:0\\]
from being modified by software meanwhile the hardware can store internally the current DA\\[6:0\\]
into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. EVR.CRF) and IBI flag (i.e. EVR.IBIF) are both cleared."]
        #[inline(always)]
        pub fn set_crack(&mut self, val: super::vals::Devr4Crack) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\]
bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received RDR. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub const fn ibiden(&self) -> super::vals::Devr4Ibiden {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Devr4Ibiden::from_bits(val as u8)
        }
        #[doc = "IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\\[2\\]
bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received RDR. Writing to this field has no impact when the read field DEVRx.DIS=1."]
        #[inline(always)]
        pub fn set_ibiden(&mut self, val: super::vals::Devr4Ibiden) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3’b101). If this bit is set, when an IBI is received (i.e. EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
        #[inline(always)]
        pub const fn susp(&self) -> super::vals::Devr4Susp {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Devr4Susp::from_bits(val as u8)
        }
        #[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\\[7:5\\]=3’b101). If this bit is set, when an IBI is received (i.e. EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO."]
        #[inline(always)]
        pub fn set_susp(&mut self, val: super::vals::Devr4Susp) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "DA\\[6:0\\]
write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\]
and IBIDEN values. Then, to be able to next modify DA\\[6:0\\]
or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\]
or IBIDEN."]
        #[inline(always)]
        pub const fn dis(&self) -> super::vals::Devr4Dis {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Devr4Dis::from_bits(val as u8)
        }
        #[doc = "DA\\[6:0\\]
write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\\[6:0\\]
and IBIDEN values. Then, to be able to next modify DA\\[6:0\\]
or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\\[6:0\\]
or IBIDEN."]
        #[inline(always)]
        pub fn set_dis(&mut self, val: super::vals::Devr4Dis) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Devr4 {
        #[inline(always)]
        fn default() -> Devr4 {
            Devr4(0)
        }
    }
    #[doc = "I3C extended provisioned ID register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epidr(pub u32);
    impl Epidr {
        #[doc = "4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\\[15:12\\]
of the 48-bit provisioned ID. Note: The bits\\[11:0\\]
of the provisioned ID may be 0."]
        #[inline(always)]
        pub const fn mipiid(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "4-bit MIPI Instance ID This field is written by software to set and identify individually each instance of this I3C IP with a specific number on a single I3C bus. This field represents the bits\\[15:12\\]
of the 48-bit provisioned ID. Note: The bits\\[11:0\\]
of the provisioned ID may be 0."]
        #[inline(always)]
        pub fn set_mipiid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "provisioned ID type selector This field is set as 0 i.e. vendor fixed value. This field represents the bit\\[32\\]
of the 48-bit provisioned ID. Note: The bits\\[31:16\\]
of the provisioned ID may be 0."]
        #[inline(always)]
        pub const fn idtsel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "provisioned ID type selector This field is set as 0 i.e. vendor fixed value. This field represents the bit\\[32\\]
of the 48-bit provisioned ID. Note: The bits\\[31:16\\]
of the provisioned ID may be 0."]
        #[inline(always)]
        pub fn set_idtsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "15-bit MIPI manufacturer ID This read field is the 15-bit STMicroelectronics MIPI ID i.e. 0x0104. This field represents the bits\\[47:33\\]
of the 48-bit provisioned ID."]
        #[inline(always)]
        pub const fn mipimid(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x7fff;
            val as u16
        }
        #[doc = "15-bit MIPI manufacturer ID This read field is the 15-bit STMicroelectronics MIPI ID i.e. 0x0104. This field represents the bits\\[47:33\\]
of the 48-bit provisioned ID."]
        #[inline(always)]
        pub fn set_mipimid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
        }
    }
    impl Default for Epidr {
        #[inline(always)]
        fn default() -> Epidr {
            Epidr(0)
        }
    }
    #[doc = "I3C event register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evr(pub u32);
    impl Evr {
        #[doc = "C-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the C-FIFO is empty when controller, and that the CR register contains no control word (i.e. none IBI/CR/HJ request) when target. This flag is de-asserted by hardware to indicate that the C-FIFO is not empty when controller, and that the CR register contains one control word (i.e. a pending IBI/CR/HJ request) when target. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer."]
        #[inline(always)]
        pub const fn cfef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the C-FIFO is empty when controller, and that the CR register contains no control word (i.e. none IBI/CR/HJ request) when target. This flag is de-asserted by hardware to indicate that the C-FIFO is not empty when controller, and that the CR register contains one control word (i.e. a pending IBI/CR/HJ request) when target. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer."]
        #[inline(always)]
        pub fn set_cfef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the TX-FIFO is empty. This flag is de-asserted by hardware to indicate that the TX-FIFO is not empty. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer."]
        #[inline(always)]
        pub const fn txfef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the TX-FIFO is empty. This flag is de-asserted by hardware to indicate that the TX-FIFO is not empty. Note: When the I3C is acting as controller, if the C-FIFO and TX-FIFO preload is configured (i.e. CFGR.TMODE=1), the software must wait for TXFEF=1 and CFEF=1 before starting a new frame transfer."]
        #[inline(always)]
        pub fn set_txfef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "C-FIFO not full flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a control word is to be written to the C-FIFO. This flag is de-asserted by hardware to indicate that a control word is not to be written to the C-FIFO. Note: The software must wait for CFNFF=1 (by polling or via the enabled interrupt) before writing to C-FIFO (i.e. writing to CR)."]
        #[inline(always)]
        pub const fn cfnff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO not full flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a control word is to be written to the C-FIFO. This flag is de-asserted by hardware to indicate that a control word is not to be written to the C-FIFO. Note: The software must wait for CFNFF=1 (by polling or via the enabled interrupt) before writing to C-FIFO (i.e. writing to CR)."]
        #[inline(always)]
        pub fn set_cfnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "S-FIFO not empty flag (when the I3C is acting as controller) When the I3C is acting as controller, if the S-FIFO is enabled (i.e. CFGR.RMODE=1), this flag is asserted by hardware to indicate that a status word is to be read from the S-FIFO. This flag is de-asserted by hardware to indicate that a status word is not to be read from the S-FIFO."]
        #[inline(always)]
        pub const fn sfnef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "S-FIFO not empty flag (when the I3C is acting as controller) When the I3C is acting as controller, if the S-FIFO is enabled (i.e. CFGR.RMODE=1), this flag is asserted by hardware to indicate that a status word is to be read from the S-FIFO. This flag is de-asserted by hardware to indicate that a status word is not to be read from the S-FIFO."]
        #[inline(always)]
        pub fn set_sfnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX-FIFO not full flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte/word is to be written to the TX-FIFO. This flag is de-asserted by hardware to indicate that a data byte/word is not to be written to the TX-FIFO. Note: The software must wait for TXFNFF=1 (by polling or via the enabled interrupt) before writing to TX-FIFO (i.e. writing to TDR or TDWR depending on CFGR.TXTHRES). Note: When the I3C is acting as target, if the software intends to use the TXFNFF flag for writing into TDR/TDWR, it must have configured and set the TX-FIFO preload (i.e. write TGTTDR.PRELOAD)."]
        #[inline(always)]
        pub const fn txfnff(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO not full flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte/word is to be written to the TX-FIFO. This flag is de-asserted by hardware to indicate that a data byte/word is not to be written to the TX-FIFO. Note: The software must wait for TXFNFF=1 (by polling or via the enabled interrupt) before writing to TX-FIFO (i.e. writing to TDR or TDWR depending on CFGR.TXTHRES). Note: When the I3C is acting as target, if the software intends to use the TXFNFF flag for writing into TDR/TDWR, it must have configured and set the TX-FIFO preload (i.e. write TGTTDR.PRELOAD)."]
        #[inline(always)]
        pub fn set_txfnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RX-FIFO not empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte is to be read from the RX-FIFO. This flag is de-asserted by hardware to indicate that a data byte is not to be read from the RX-FIFO. Note: The software must wait for RXFNEF=1 (by polling or via the enabled interrupt) before reading from RX-FIFO (i.e. writing to RDR or RDWR depending on CFGR.RXTHRES)."]
        #[inline(always)]
        pub const fn rxfnef(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO not empty flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that a data byte is to be read from the RX-FIFO. This flag is de-asserted by hardware to indicate that a data byte is not to be read from the RX-FIFO. Note: The software must wait for RXFNEF=1 (by polling or via the enabled interrupt) before reading from RX-FIFO (i.e. writing to RDR or RDWR depending on CFGR.RXTHRES)."]
        #[inline(always)]
        pub fn set_rxfnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "last written data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on CFGR.TXTHRES) of a message is to be written to the TX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is written."]
        #[inline(always)]
        pub const fn txlastf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "last written data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on CFGR.TXTHRES) of a message is to be written to the TX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is written."]
        #[inline(always)]
        pub fn set_txlastf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "last read data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on CFGR.RXTHRES) of a message is to be read from the RX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is read."]
        #[inline(always)]
        pub const fn rxlastf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "last read data byte/word flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that the last data byte/word (depending on CFGR.RXTHRES) of a message is to be read from the RX-FIFO. This flag is de-asserted by hardware when the last data byte/word of a message is read."]
        #[inline(always)]
        pub fn set_rxlastf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "frame complete flag (whatever the I3C is acting as controller/target) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a frame has been (normally) completed on the I3C bus, i.e when a stop is issued. When the I3C is acting as target, this flag is asserted by hardware to indicate that a message addressed to/by this target has been (normally) completed on the I3C bus, i.e when a next stop or repeated start is then issued by the controller. This flag is cleared when software writes 1 into corresponding CEVR.CFCF bit."]
        #[inline(always)]
        pub const fn fcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "frame complete flag (whatever the I3C is acting as controller/target) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a frame has been (normally) completed on the I3C bus, i.e when a stop is issued. When the I3C is acting as target, this flag is asserted by hardware to indicate that a message addressed to/by this target has been (normally) completed on the I3C bus, i.e when a next stop or repeated start is then issued by the controller. This flag is cleared when software writes 1 into corresponding CEVR.CFCF bit."]
        #[inline(always)]
        pub fn set_fcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "target-initiated read end flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that the target has prematurely ended a read transfer. Then, software should read SR to get more information on the prematurely read transfer. This flag is cleared when software writes 1 into corresponding CEVR.CRXTGTENDF bit."]
        #[inline(always)]
        pub const fn rxtgtendf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "target-initiated read end flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that the target has prematurely ended a read transfer. Then, software should read SR to get more information on the prematurely read transfer. This flag is cleared when software writes 1 into corresponding CEVR.CRXTGTENDF bit."]
        #[inline(always)]
        pub fn set_rxtgtendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that an error occurred.Then, software should read SER to get the error type. This flag is cleared when software writes 1 into corresponding CEVR.CERRF bit."]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "flag (whatever the I3C is acting as controller/target) This flag is asserted by hardware to indicate that an error occurred.Then, software should read SER to get the error type. This flag is cleared when software writes 1 into corresponding CEVR.CERRF bit."]
        #[inline(always)]
        pub fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "IBI flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an IBI request has been received. This flag is cleared when software writes 1 into corresponding CEVR.CIBIF bit."]
        #[inline(always)]
        pub const fn ibif(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "IBI flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an IBI request has been received. This flag is cleared when software writes 1 into corresponding CEVR.CIBIF bit."]
        #[inline(always)]
        pub fn set_ibif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "IBI end flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a IBI transfer has been received and completed (IBI acknowledged and IBI data bytes read by controller if any). This flag is cleared when software writes 1 into corresponding CEVR.CIBIENDF bit."]
        #[inline(always)]
        pub const fn ibiendf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IBI end flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a IBI transfer has been received and completed (IBI acknowledged and IBI data bytes read by controller if any). This flag is cleared when software writes 1 into corresponding CEVR.CIBIENDF bit."]
        #[inline(always)]
        pub fn set_ibiendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "controller-role request flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a controller-role request has been acknowledged and completed (by hardware). The software should then issue a GETACCCR CCC (get accept controller role) for the controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding CEVR.CCRF bit."]
        #[inline(always)]
        pub const fn crf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "controller-role request flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that a controller-role request has been acknowledged and completed (by hardware). The software should then issue a GETACCCR CCC (get accept controller role) for the controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding CEVR.CCRF bit."]
        #[inline(always)]
        pub fn set_crf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "controller-role update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that it has now gained the controller role after the completed controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding CEVR.CCRUPDF bit."]
        #[inline(always)]
        pub const fn crupdf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "controller-role update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that it has now gained the controller role after the completed controller-role hand-off procedure. This flag is cleared when software writes 1 into corresponding CEVR.CCRUPDF bit."]
        #[inline(always)]
        pub fn set_crupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "hot-join flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an hot join request has been received. This flag is cleared when software writes 1 into corresponding CEVR.CHJF bit."]
        #[inline(always)]
        pub const fn hjf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "hot-join flag (when the I3C is acting as controller) When the I3C is acting as controller, this flag is asserted by hardware to indicate that an hot join request has been received. This flag is cleared when software writes 1 into corresponding CEVR.CHJF bit."]
        #[inline(always)]
        pub fn set_hjf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "wakeup/missed start flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a start has been detected (i.e. a SDA falling edge followed by a SCL falling edge) but on the next SCL falling edge, the I3C kernel clock is (still) gated. Thus an I3C bus transaction may have been lost by the target. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding CEVR.CWKPF bit."]
        #[inline(always)]
        pub const fn wkpf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup/missed start flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a start has been detected (i.e. a SDA falling edge followed by a SCL falling edge) but on the next SCL falling edge, the I3C kernel clock is (still) gated. Thus an I3C bus transaction may have been lost by the target. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding CEVR.CWKPF bit."]
        #[inline(always)]
        pub fn set_wkpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "get flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that any direct CCC of get type (GET*** CCC) has been received. This flag is cleared when software writes 1 into corresponding CEVR.CGETF bit."]
        #[inline(always)]
        pub const fn getf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "get flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that any direct CCC of get type (GET*** CCC) has been received. This flag is cleared when software writes 1 into corresponding CEVR.CGETF bit."]
        #[inline(always)]
        pub fn set_getf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "get status flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct GETSTATUS CCC (get status) has been received. This flag is cleared when software writes 1 into corresponding CEVR.CSTAF bit."]
        #[inline(always)]
        pub const fn staf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "get status flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct GETSTATUS CCC (get status) has been received. This flag is cleared when software writes 1 into corresponding CEVR.CSTAF bit."]
        #[inline(always)]
        pub fn set_staf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "dynamic address update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a dynamic address update has been received via any of the broadcast ENTDAA, RSTDAA and direct SETNEWDA CCC. Then, software should read DEVR0.DA\\[6:0\\]
to get the maximum write length value. This flag is cleared when software writes 1 into corresponding CEVR.CDAUPDF bit."]
        #[inline(always)]
        pub const fn daupdf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "dynamic address update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a dynamic address update has been received via any of the broadcast ENTDAA, RSTDAA and direct SETNEWDA CCC. Then, software should read DEVR0.DA\\[6:0\\]
to get the maximum write length value. This flag is cleared when software writes 1 into corresponding CEVR.CDAUPDF bit."]
        #[inline(always)]
        pub fn set_daupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "maximum write length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMWL CCC (set max write length) has been received. Then, software should read MAXWLR.MWL\\[15:0\\]
to get the maximum write length value. This flag is cleared when software writes 1 into corresponding CEVR.CMWLUPDF bit."]
        #[inline(always)]
        pub const fn mwlupdf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "maximum write length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMWL CCC (set max write length) has been received. Then, software should read MAXWLR.MWL\\[15:0\\]
to get the maximum write length value. This flag is cleared when software writes 1 into corresponding CEVR.CMWLUPDF bit."]
        #[inline(always)]
        pub fn set_mwlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "maximum read length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMRL CCC (set max read length) has been received. Then, software should read MAXRLR.MRL\\[15:0\\]
to get the maximum read length value. This flag is cleared when software writes 1 into corresponding CEVR.CMRLUPDF bit."]
        #[inline(always)]
        pub const fn mrlupdf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "maximum read length update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a direct SETMRL CCC (set max read length) has been received. Then, software should read MAXRLR.MRL\\[15:0\\]
to get the maximum read length value. This flag is cleared when software writes 1 into corresponding CEVR.CMRLUPDF bit."]
        #[inline(always)]
        pub fn set_mrlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "reset pattern flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a reset pattern has been detected (i.e. 14 SDA transitions while SCL is low, followed by repeated start, then stop). Then, software should read DEVR0.RSTACT\\[1:0\\]
and DEVR0.RSTVAL, to know what reset level is required. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), DEVR0.RSTACT\\[1:0\\]
dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding CEVR.CRSTF bit."]
        #[inline(always)]
        pub const fn rstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "reset pattern flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that a reset pattern has been detected (i.e. 14 SDA transitions while SCL is low, followed by repeated start, then stop). Then, software should read DEVR0.RSTACT\\[1:0\\]
and DEVR0.RSTVAL, to know what reset level is required. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), DEVR0.RSTACT\\[1:0\\]
dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one. The corresponding interrupt may be used to wakeup the device from a low power mode (Sleep or Stop mode). This flag is cleared when software writes 1 into corresponding CEVR.CRSTF bit."]
        #[inline(always)]
        pub fn set_rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "activity state update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENTASx CCC (with x=0...3) has been received. Then, software should read DEVR0.AS\\[1:0\\]. This flag is cleared when software writes 1 into corresponding CEVR.CASUPDF bit."]
        #[inline(always)]
        pub const fn asupdf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "activity state update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENTASx CCC (with x=0...3) has been received. Then, software should read DEVR0.AS\\[1:0\\]. This flag is cleared when software writes 1 into corresponding CEVR.CASUPDF bit."]
        #[inline(always)]
        pub fn set_asupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "interrupt/controller-role/hot-join update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENEC/DISEC CCC (enable/disable target events) has been received, where a target event is either an interrupt/IBI request, a controller-role request, or an hot-join request. Then, software should read respectively DEVR0.IBIEN, DEVR0.CREN or DEVR0.HJEN. This flag is cleared when software writes 1 into corresponding CEVR.CINTUPDF bit."]
        #[inline(always)]
        pub const fn intupdf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt/controller-role/hot-join update flag (when the I3C is acting as target) When the I3C is acting as target, this flag is asserted by hardware to indicate that the direct or broadcast ENEC/DISEC CCC (enable/disable target events) has been received, where a target event is either an interrupt/IBI request, a controller-role request, or an hot-join request. Then, software should read respectively DEVR0.IBIEN, DEVR0.CREN or DEVR0.HJEN. This flag is cleared when software writes 1 into corresponding CEVR.CINTUPDF bit."]
        #[inline(always)]
        pub fn set_intupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "DEFTGTS flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFTGTS CCC (define list of targets) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding CEVR.CDEFF bit."]
        #[inline(always)]
        pub const fn deff(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DEFTGTS flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFTGTS CCC (define list of targets) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding CEVR.CDEFF bit."]
        #[inline(always)]
        pub fn set_deff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "group addressing flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFGRPA CCC (define list of group addresses) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding CEVR.CGRPF bit."]
        #[inline(always)]
        pub const fn grpf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "group addressing flag (when the I3C is acting as target) When the I3C is acting as target (and is typically controller capable), this flag is asserted by hardware to indicate that the broadcast DEFGRPA CCC (define list of group addresses) has been received. Then, software may store the received data for when getting the controller role. This flag is cleared when software writes 1 into corresponding CEVR.CGRPF bit."]
        #[inline(always)]
        pub fn set_grpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Evr {
        #[inline(always)]
        fn default() -> Evr {
            Evr(0)
        }
    }
    #[doc = "I3C get capability register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Getcapr(pub u32);
    impl Getcapr {
        #[doc = "IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\]
value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1."]
        #[inline(always)]
        pub const fn cappend(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\\[7:0\\]
value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1."]
        #[inline(always)]
        pub fn set_cappend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Getcapr {
        #[inline(always)]
        fn default() -> Getcapr {
            Getcapr(0)
        }
    }
    #[doc = "I3C get capability register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Getmxdsr(pub u32);
    impl Getmxdsr {
        #[doc = "controller hand-off activity state This bit is written by software during bus initialization (i.e. CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership."]
        #[inline(always)]
        pub const fn hoffas(&self) -> super::vals::Hoffas {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Hoffas::from_bits(val as u8)
        }
        #[doc = "controller hand-off activity state This bit is written by software during bus initialization (i.e. CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership."]
        #[inline(always)]
        pub fn set_hoffas(&mut self, val: super::vals::Hoffas) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "GETMXDS CCC format This field is written by software during bus initialization (i.e. CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\\[7:0\\]. - Max read turnaround time is less than 256 �s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\\[7:0\\]
and LSB=0. - Max read turnaround time is between 256 �s and 65535 �s - 3-byte MaxRdTurn is returned with MSB=RDTURN\\[7:0\\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 �s and 16 s."]
        #[inline(always)]
        pub const fn fmt(&self) -> super::vals::Fmt {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Fmt::from_bits(val as u8)
        }
        #[doc = "GETMXDS CCC format This field is written by software during bus initialization (i.e. CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\\[7:0\\]. - Max read turnaround time is less than 256 �s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\\[7:0\\]
and LSB=0. - Max read turnaround time is between 256 �s and 65535 �s - 3-byte MaxRdTurn is returned with MSB=RDTURN\\[7:0\\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 �s and 16 s."]
        #[inline(always)]
        pub fn set_fmt(&mut self, val: super::vals::Fmt) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. CFGR.EN=0) and writes the value of the selected byte (via the FMT\\[1:0\\]
field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time."]
        #[inline(always)]
        pub const fn rdturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. CFGR.EN=0) and writes the value of the selected byte (via the FMT\\[1:0\\]
field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time."]
        #[inline(always)]
        pub fn set_rdturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\]
bits."]
        #[inline(always)]
        pub const fn tsco(&self) -> super::vals::Tsco {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Tsco::from_bits(val as u8)
        }
        #[doc = "clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\\[5:3\\]
bits."]
        #[inline(always)]
        pub fn set_tsco(&mut self, val: super::vals::Tsco) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Getmxdsr {
        #[inline(always)]
        fn default() -> Getmxdsr {
            Getmxdsr(0)
        }
    }
    #[doc = "I3C IBI payload data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ibidr(pub u32);
    impl Ibidr {
        #[doc = "8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\\[7:0\\]
mandatory data byte)."]
        #[inline(always)]
        pub const fn ibidb0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit IBI payload data (earliest byte on I3C bus, i.e. MDB\\[7:0\\]
mandatory data byte)."]
        #[inline(always)]
        pub fn set_ibidb0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "8-bit IBI payload data (next byte on I3C bus after IBIDB0\\[7:0\\])."]
        #[inline(always)]
        pub const fn ibidb1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit IBI payload data (next byte on I3C bus after IBIDB0\\[7:0\\])."]
        #[inline(always)]
        pub fn set_ibidb1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "8-bit IBI payload data (next byte on I3C bus after IBIDB1\\[7:0\\])."]
        #[inline(always)]
        pub const fn ibidb2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit IBI payload data (next byte on I3C bus after IBIDB1\\[7:0\\])."]
        #[inline(always)]
        pub fn set_ibidb2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "8-bit IBI payload data (latest byte on I3C bus)."]
        #[inline(always)]
        pub const fn ibidb3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit IBI payload data (latest byte on I3C bus)."]
        #[inline(always)]
        pub fn set_ibidb3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Ibidr {
        #[inline(always)]
        fn default() -> Ibidr {
            Ibidr(0)
        }
    }
    #[doc = "I3C interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub const fn cfnfie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub fn set_cfnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub const fn sfneie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub fn set_sfneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub const fn txfnfie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub fn set_txfnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub const fn rxfneie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub fn set_rxfneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "frame complete interrupt enable (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub const fn fcie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "frame complete interrupt enable (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub fn set_fcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "target-initiated read end interrupt enable (when the I3C is acting as controller)"]
        #[inline(always)]
        pub const fn rxtgtendie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "target-initiated read end interrupt enable (when the I3C is acting as controller)"]
        #[inline(always)]
        pub fn set_rxtgtendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error interrupt enable (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error interrupt enable (whatever the I3C is acting as controller/target)"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "IBI request interrupt enable (when the I3C is acting as controller)"]
        #[inline(always)]
        pub const fn ibiie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "IBI request interrupt enable (when the I3C is acting as controller)"]
        #[inline(always)]
        pub fn set_ibiie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "IBI end interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn ibiendie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IBI end interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_ibiendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "controller-role request interrupt enable (when the I3C is acting as controller)"]
        #[inline(always)]
        pub const fn crie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "controller-role request interrupt enable (when the I3C is acting as controller)"]
        #[inline(always)]
        pub fn set_crie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "controller-role update interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn crupdie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "controller-role update interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_crupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "hot-join interrupt enable (when the I3C is acting as controller)"]
        #[inline(always)]
        pub const fn hjie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "hot-join interrupt enable (when the I3C is acting as controller)"]
        #[inline(always)]
        pub fn set_hjie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "wakeup interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn wkpie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_wkpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "GETxxx CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn getie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "GETxxx CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_getie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "GETSTATUS CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn staie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "GETSTATUS CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_staie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn daupdie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_daupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SETMWL CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn mwlupdie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SETMWL CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_mwlupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "SETMRL CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn mrlupdie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SETMRL CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_mrlupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "reset pattern interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn rstie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "reset pattern interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_rstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ENTASx CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn asupdie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ENTASx CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_asupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn intupdie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_intupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "DEFTGTS CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn defie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DEFTGTS CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_defie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "DEFGRPA CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn grpie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "DEFGRPA CCC interrupt enable (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_grpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "I3C maximum read length register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maxrlr(pub u32);
    impl Maxrlr {
        #[doc = "maximum data read length (when I3C is acting as target) This field is initially written by software when CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC."]
        #[inline(always)]
        pub const fn mrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "maximum data read length (when I3C is acting as target) This field is initially written by software when CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC."]
        #[inline(always)]
        pub fn set_mrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100"]
        #[inline(always)]
        pub const fn ibip(&self) -> super::vals::Ibip {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Ibip::from_bits(val as u8)
        }
        #[doc = "IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\\[2:0\\]). Software is notified of a MRL update by the EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100"]
        #[inline(always)]
        pub fn set_ibip(&mut self, val: super::vals::Ibip) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Maxrlr {
        #[inline(always)]
        fn default() -> Maxrlr {
            Maxrlr(0)
        }
    }
    #[doc = "I3C maximum write length register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maxwlr(pub u32);
    impl Maxwlr {
        #[doc = "maximum data write length (when I3C is acting as target) This field is initially written by software when CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
        #[inline(always)]
        pub const fn mwl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "maximum data write length (when I3C is acting as target) This field is initially written by software when CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
        #[inline(always)]
        pub fn set_mwl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Maxwlr {
        #[inline(always)]
        fn default() -> Maxwlr {
            Maxwlr(0)
        }
    }
    #[doc = "I3C receive data byte register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdr(pub u32);
    impl Rdr {
        #[doc = "8-bit received data on I3C bus."]
        #[inline(always)]
        pub const fn rdb0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit received data on I3C bus."]
        #[inline(always)]
        pub fn set_rdb0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rdr {
        #[inline(always)]
        fn default() -> Rdr {
            Rdr(0)
        }
    }
    #[doc = "I3C receive data word register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdwr(pub u32);
    impl Rdwr {
        #[doc = "8-bit received data (earliest byte on I3C bus)."]
        #[inline(always)]
        pub const fn rdb0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit received data (earliest byte on I3C bus)."]
        #[inline(always)]
        pub fn set_rdb0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "8-bit received data (next byte after RDB0 on I3C bus)."]
        #[inline(always)]
        pub const fn rdb1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit received data (next byte after RDB0 on I3C bus)."]
        #[inline(always)]
        pub fn set_rdb1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "8-bit received data (next byte after RDB1 on I3C bus)."]
        #[inline(always)]
        pub const fn rdb2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit received data (next byte after RDB1 on I3C bus)."]
        #[inline(always)]
        pub fn set_rdb2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "8-bit received data (latest byte on I3C bus)."]
        #[inline(always)]
        pub const fn rdb3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit received data (latest byte on I3C bus)."]
        #[inline(always)]
        pub fn set_rdb3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Rdwr {
        #[inline(always)]
        fn default() -> Rdwr {
            Rdwr(0)
        }
    }
    #[doc = "I3C received message register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rmr(pub u32);
    impl Rmr {
        #[doc = "IBI received payload data count (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the number of data bytes effectively received in the IBIDR register."]
        #[inline(always)]
        pub const fn ibirdcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "IBI received payload data count (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the number of data bytes effectively received in the IBIDR register."]
        #[inline(always)]
        pub fn set_ibirdcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "received CCC code (when the I3C is configured as target) When the I3C is configured as target, this field logs the received CCC code."]
        #[inline(always)]
        pub const fn rcode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "received CCC code (when the I3C is configured as target) When the I3C is configured as target, this field logs the received CCC code."]
        #[inline(always)]
        pub fn set_rcode(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "received target address (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the received dynamic address from the target during acknowledged IBI or controller-role request."]
        #[inline(always)]
        pub const fn radd(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "received target address (when the I3C is configured as controller) When the I3C is configured as controller, this field logs the received dynamic address from the target during acknowledged IBI or controller-role request."]
        #[inline(always)]
        pub fn set_radd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
    }
    impl Default for Rmr {
        #[inline(always)]
        fn default() -> Rmr {
            Rmr(0)
        }
    }
    #[doc = "I3C status error register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ser(pub u32);
    impl Ser {
        #[doc = "protocol error code/type controller detected an illegally formatted CCC controller detected that transmitted data on the bus is different from expected controller detected a not acknowledged broadcast address (7’hE) controller detected the new controller did not drive bus after controller-role hand-off target detected an invalid broadcast address 7’hE+W target detected a parity error on a CCC code via a parity check (vs T bit) target detected a parity error on a write data via a parity check (vs T bit) target detected a parity error on the assigned address during dynamic address arbitration via a parity check (vs PAR bit) target detected a 7’hE+R missing after Sr during dynamic address arbitration target detected an illegally formatted CCC target detected that transmitted data on the bus is different from expected others: reserved"]
        #[inline(always)]
        pub const fn coderr(&self) -> super::vals::Coderr {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Coderr::from_bits(val as u8)
        }
        #[doc = "protocol error code/type controller detected an illegally formatted CCC controller detected that transmitted data on the bus is different from expected controller detected a not acknowledged broadcast address (7’hE) controller detected the new controller did not drive bus after controller-role hand-off target detected an invalid broadcast address 7’hE+W target detected a parity error on a CCC code via a parity check (vs T bit) target detected a parity error on a write data via a parity check (vs T bit) target detected a parity error on the assigned address during dynamic address arbitration via a parity check (vs PAR bit) target detected a 7’hE+R missing after Sr during dynamic address arbitration target detected an illegally formatted CCC target detected that transmitted data on the bus is different from expected others: reserved"]
        #[inline(always)]
        pub fn set_coderr(&mut self, val: super::vals::Coderr) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "protocol error"]
        #[inline(always)]
        pub const fn perr(&self) -> super::vals::Perr {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Perr::from_bits(val as u8)
        }
        #[doc = "protocol error"]
        #[inline(always)]
        pub fn set_perr(&mut self, val: super::vals::Perr) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "SCL stall error (when the I3C is acting as target)"]
        #[inline(always)]
        pub const fn stall(&self) -> super::vals::Stall {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Stall::from_bits(val as u8)
        }
        #[doc = "SCL stall error (when the I3C is acting as target)"]
        #[inline(always)]
        pub fn set_stall(&mut self, val: super::vals::Stall) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "RX-FIFO overrun or TX-FIFO underrun i) a TX-FIFO underrun: TX-FIFO is empty and a write data byte has to be transmitted ii) a RX-FIFO overrun: RX-FIFO is full and a new data byte is received"]
        #[inline(always)]
        pub const fn dovr(&self) -> super::vals::Dovr {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Dovr::from_bits(val as u8)
        }
        #[doc = "RX-FIFO overrun or TX-FIFO underrun i) a TX-FIFO underrun: TX-FIFO is empty and a write data byte has to be transmitted ii) a RX-FIFO overrun: RX-FIFO is full and a new data byte is received"]
        #[inline(always)]
        pub fn set_dovr(&mut self, val: super::vals::Dovr) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "C-FIFO underrun or S-FIFO overrun (when the I3C is acting as controller) i) a C-FIFO underrun: control FIFO is empty and a restart has to be emitted ii) a S-FIFO overrun: S-FIFO is full and a new message ends"]
        #[inline(always)]
        pub const fn covr(&self) -> super::vals::Covr {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Covr::from_bits(val as u8)
        }
        #[doc = "C-FIFO underrun or S-FIFO overrun (when the I3C is acting as controller) i) a C-FIFO underrun: control FIFO is empty and a restart has to be emitted ii) a S-FIFO overrun: S-FIFO is full and a new message ends"]
        #[inline(always)]
        pub fn set_covr(&mut self, val: super::vals::Covr) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "address not acknowledged (when the I3C is configured as controller) i) a legacy I2C read/write transfer ii) a direct CCC write transfer iii) the second trial of a direct CCC read transfer iv) a private read/write transfer"]
        #[inline(always)]
        pub const fn anack(&self) -> super::vals::Anack {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Anack::from_bits(val as u8)
        }
        #[doc = "address not acknowledged (when the I3C is configured as controller) i) a legacy I2C read/write transfer ii) a direct CCC write transfer iii) the second trial of a direct CCC read transfer iv) a private read/write transfer"]
        #[inline(always)]
        pub fn set_anack(&mut self, val: super::vals::Anack) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "data not acknowledged (when the I3C is acting as controller) i) a legacy I2C write transfer ii) the second trial when sending dynamic address during ENTDAA procedure"]
        #[inline(always)]
        pub const fn dnack(&self) -> super::vals::Dnack {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Dnack::from_bits(val as u8)
        }
        #[doc = "data not acknowledged (when the I3C is acting as controller) i) a legacy I2C write transfer ii) the second trial when sending dynamic address during ENTDAA procedure"]
        #[inline(always)]
        pub fn set_dnack(&mut self, val: super::vals::Dnack) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "data error (when the I3C is acting as controller)"]
        #[inline(always)]
        pub const fn derr(&self) -> super::vals::Derr {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Derr::from_bits(val as u8)
        }
        #[doc = "data error (when the I3C is acting as controller)"]
        #[inline(always)]
        pub fn set_derr(&mut self, val: super::vals::Derr) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Ser {
        #[inline(always)]
        fn default() -> Ser {
            Ser(0)
        }
    }
    #[doc = "I3C status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "data counter - When the I3C is acting as controller: number of targets detected on the bus - When the I3C is acting as target: number of transmitted bytes - Whatever the I3C is acting as controller or target: number of data bytes read from or transmitted on the I3C bus during the MID\\[7:0\\]
message"]
        #[inline(always)]
        pub const fn xdcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "data counter - When the I3C is acting as controller: number of targets detected on the bus - When the I3C is acting as target: number of transmitted bytes - Whatever the I3C is acting as controller or target: number of data bytes read from or transmitted on the I3C bus during the MID\\[7:0\\]
message"]
        #[inline(always)]
        pub fn set_xdcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "a private read message is completed/aborted prematurely by the target (when the I3C is acting as controller) When the I3C is acting as controller, this bit indicates if the private read data which is transmitted by the target early terminates (i.e. the target drives T bit low earlier vs what does expect the controller in terms of programmed number of read data bytes i.e. CR.DCNT\\[15:0\\])."]
        #[inline(always)]
        pub const fn abt(&self) -> super::vals::Abt {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Abt::from_bits(val as u8)
        }
        #[doc = "a private read message is completed/aborted prematurely by the target (when the I3C is acting as controller) When the I3C is acting as controller, this bit indicates if the private read data which is transmitted by the target early terminates (i.e. the target drives T bit low earlier vs what does expect the controller in terms of programmed number of read data bytes i.e. CR.DCNT\\[15:0\\])."]
        #[inline(always)]
        pub fn set_abt(&mut self, val: super::vals::Abt) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "message direction Whatever the I3C is acting as controller or target, this bit indicates the direction of the related message on the I3C bus Note: ENTDAA CCC is considered as a write command."]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "message direction Whatever the I3C is acting as controller or target, this bit indicates the direction of the related message on the I3C bus Note: ENTDAA CCC is considered as a write command."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "message identifier/counter of a given frame (when the I3C is acting as controller) When the I3C is acting as controller, this field identifies the control word message (i.e. CR) to which the SR status register refers. First message of a frame is identified with MID\\[7:0\\]=0. This field is incremented (by hardware) on the completion of a new message control word (i.e. CR) over I3C bus. This field is reset for every new frame start."]
        #[inline(always)]
        pub const fn mid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "message identifier/counter of a given frame (when the I3C is acting as controller) When the I3C is acting as controller, this field identifies the control word message (i.e. CR) to which the SR status register refers. First message of a frame is identified with MID\\[7:0\\]=0. This field is incremented (by hardware) on the completion of a new message control word (i.e. CR) over I3C bus. This field is reset for every new frame start."]
        #[inline(always)]
        pub fn set_mid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    #[doc = "I3C transmit data byte register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdr(pub u32);
    impl Tdr {
        #[doc = "8-bit data to transmit on I3C bus."]
        #[inline(always)]
        pub const fn tdb0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit data to transmit on I3C bus."]
        #[inline(always)]
        pub fn set_tdb0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Tdr {
        #[inline(always)]
        fn default() -> Tdr {
            Tdr(0)
        }
    }
    #[doc = "I3C transmit data word register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdwr(pub u32);
    impl Tdwr {
        #[doc = "8-bit transmit data (earliest byte on I3C bus)"]
        #[inline(always)]
        pub const fn tdb0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit transmit data (earliest byte on I3C bus)"]
        #[inline(always)]
        pub fn set_tdb0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "8-bit transmit data (next byte after TDB0\\[7:0\\]
on I3C bus)."]
        #[inline(always)]
        pub const fn tdb1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit transmit data (next byte after TDB0\\[7:0\\]
on I3C bus)."]
        #[inline(always)]
        pub fn set_tdb1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "8-bit transmit data (next byte after TDB1\\[7:0\\]
on I3C bus)."]
        #[inline(always)]
        pub const fn tdb2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit transmit data (next byte after TDB1\\[7:0\\]
on I3C bus)."]
        #[inline(always)]
        pub fn set_tdb2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "8-bit transmit data (latest byte on I3C bus)."]
        #[inline(always)]
        pub const fn tdb3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit transmit data (latest byte on I3C bus)."]
        #[inline(always)]
        pub fn set_tdb3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Tdwr {
        #[inline(always)]
        fn default() -> Tdwr {
            Tdwr(0)
        }
    }
    #[doc = "I3C target transmit configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tgttdr(pub u32);
    impl Tgttdr {
        #[doc = "transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO."]
        #[inline(always)]
        pub const fn tgttdcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO."]
        #[inline(always)]
        pub fn set_tgttdcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO."]
        #[inline(always)]
        pub const fn preload(&self) -> super::vals::Preload {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Preload::from_bits(val as u8)
        }
        #[doc = "preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO."]
        #[inline(always)]
        pub fn set_preload(&mut self, val: super::vals::Preload) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Tgttdr {
        #[inline(always)]
        fn default() -> Tgttdr {
            Tgttdr(0)
        }
    }
    #[doc = "I3C timing register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr0(pub u32);
    impl Timingr0 {
        #[doc = "SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
        #[inline(always)]
        pub const fn scll_pp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
        #[inline(always)]
        pub fn set_scll_pp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
        #[inline(always)]
        pub const fn sclh_i3c(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
        #[inline(always)]
        pub fn set_sclh_i3c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
        #[inline(always)]
        pub const fn scll_od(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
        #[inline(always)]
        pub fn set_scll_od(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
        #[inline(always)]
        pub const fn sclh_i2c(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
        #[inline(always)]
        pub fn set_sclh_i2c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Timingr0 {
        #[inline(always)]
        fn default() -> Timingr0 {
            Timingr0(0)
        }
    }
    #[doc = "I3C timing register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr1(pub u32);
    impl Timingr1 {
        #[doc = "number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 �s. This timing is defined by: tAVAL = (AVAL\\[7:0\\]
+ 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 �s . This timing is defined by: tIDLE = (AVAL\\[7:0\\]
+ 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\\[7:0\\]
+ 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 �s. This timing is defined by: tSTALL = (AVAL\\[7:0\\]
+ 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined TIMINGR1.ANSCR\\[1:0\\], as follows: If ASNCR\\[1:0\\]=00: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x tI3CCLK If ASNCR\\[1:0\\]=01: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x 100 x tI3CCLK If ASNCR\\[1:0\\]=10: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x 2000 x tI3CCLK If ASNCR\\[1:0\\]=11: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x 50000 x tI3CCLK"]
        #[inline(always)]
        pub const fn aval(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 �s. This timing is defined by: tAVAL = (AVAL\\[7:0\\]
+ 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 �s . This timing is defined by: tIDLE = (AVAL\\[7:0\\]
+ 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\\[7:0\\]
+ 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 �s. This timing is defined by: tSTALL = (AVAL\\[7:0\\]
+ 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined TIMINGR1.ANSCR\\[1:0\\], as follows: If ASNCR\\[1:0\\]=00: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x tI3CCLK If ASNCR\\[1:0\\]=01: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x 100 x tI3CCLK If ASNCR\\[1:0\\]=10: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x 2000 x tI3CCLK If ASNCR\\[1:0\\]=11: tNEWCRLock = (AVAL\\[7:0\\]
+ 1) x 50000 x tI3CCLK"]
        #[inline(always)]
        pub fn set_aval(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "activity state of the new controller (when I3C is acting as - active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\\[7:0\\]. This field can be modified only when the I3C is acting as controller."]
        #[inline(always)]
        pub const fn asncr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "activity state of the new controller (when I3C is acting as - active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\\[7:0\\]. This field can be modified only when the I3C is acting as controller."]
        #[inline(always)]
        pub fn set_asncr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \\[ (FREE\\[6:0\\]
+ 1) x 2 - (0,5 + SDA_HD)\\]
x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1�s, 100�s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 �s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 �s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \\[ (FREE\\[6:0\\]
+ 1) x 2 - (0,5 + SDA_HD)\\]
x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\\[6:0\\]
+ 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\\[6:0\\]
+ 1) x tI3CCLK"]
        #[inline(always)]
        pub const fn free(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \\[ (FREE\\[6:0\\]
+ 1) x 2 - (0,5 + SDA_HD)\\]
x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1�s, 100�s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 �s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 �s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \\[ (FREE\\[6:0\\]
+ 1) x 2 - (0,5 + SDA_HD)\\]
x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\\[6:0\\]
+ 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\\[6:0\\]
+ 1) x tI3CCLK"]
        #[inline(always)]
        pub fn set_free(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):"]
        #[inline(always)]
        pub const fn sda_hd(&self) -> super::vals::SdaHd {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::SdaHd::from_bits(val as u8)
        }
        #[doc = "SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):"]
        #[inline(always)]
        pub fn set_sda_hd(&mut self, val: super::vals::SdaHd) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Timingr1 {
        #[inline(always)]
        fn default() -> Timingr1 {
            Timingr1(0)
        }
    }
    #[doc = "I3C timing register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr2(pub u32);
    impl Timingr2 {
        #[doc = "Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent."]
        #[inline(always)]
        pub const fn stallt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent."]
        #[inline(always)]
        pub fn set_stallt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data."]
        #[inline(always)]
        pub const fn stalld(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data."]
        #[inline(always)]
        pub fn set_stalld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command."]
        #[inline(always)]
        pub const fn stallc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command."]
        #[inline(always)]
        pub fn set_stallc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt."]
        #[inline(always)]
        pub const fn stalla(&self) -> super::vals::Stalla {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Stalla::from_bits(val as u8)
        }
        #[doc = "controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt."]
        #[inline(always)]
        pub fn set_stalla(&mut self, val: super::vals::Stalla) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK"]
        #[inline(always)]
        pub const fn stall(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK"]
        #[inline(always)]
        pub fn set_stall(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Timingr2 {
        #[inline(always)]
        fn default() -> Timingr2 {
            Timingr2(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Abt {
        #[doc = "no early completion/abort from the target"]
        B_0X0 = 0,
        #[doc = "early completion/abort from the target"]
        B_0X1 = 0x01,
    }
    impl Abt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Abt {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Abt {
        #[inline(always)]
        fn from(val: u8) -> Abt {
            Abt::from_bits(val)
        }
    }
    impl From<Abt> for u8 {
        #[inline(always)]
        fn from(val: Abt) -> u8 {
            Abt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Anack {
        #[doc = "no detected error"]
        B_0X0 = 0,
        #[doc = "controller detected that the static/dynamic address was not acknowledged by a target, either during:"]
        B_0X1 = 0x01,
    }
    impl Anack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Anack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Anack {
        #[inline(always)]
        fn from(val: u8) -> Anack {
            Anack::from_bits(val)
        }
    }
    impl From<Anack> for u8 {
        #[inline(always)]
        fn from(val: Anack) -> u8 {
            Anack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum As {
        #[doc = "activity state 0"]
        B_0X0 = 0,
        #[doc = "activity state 1"]
        B_0X1 = 0x01,
        #[doc = "activity state 2"]
        B_0X2 = 0x02,
        #[doc = "activity state 3"]
        B_0X3 = 0x03,
    }
    impl As {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> As {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for As {
        #[inline(always)]
        fn from(val: u8) -> As {
            As::from_bits(val)
        }
    }
    impl From<As> for u8 {
        #[inline(always)]
        fn from(val: As) -> u8 {
            As::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Asupdie {
        #[doc = "interrupt disabled"]
        B_0X0 = 0,
        #[doc = "interrupt enabled"]
        B_0X1 = 0x01,
    }
    impl Asupdie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Asupdie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Asupdie {
        #[inline(always)]
        fn from(val: u8) -> Asupdie {
            Asupdie::from_bits(val)
        }
    }
    impl From<Asupdie> for u8 {
        #[inline(always)]
        fn from(val: Asupdie) -> u8 {
            Asupdie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Capdhoff {
        #[doc = "this I3C does not needs additional time to process a controller-role hand-off"]
        B_0X0 = 0,
        #[doc = "this I3C needs additional time to process a controller-role hand-off"]
        B_0X1 = 0x01,
    }
    impl Capdhoff {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Capdhoff {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Capdhoff {
        #[inline(always)]
        fn from(val: u8) -> Capdhoff {
            Capdhoff::from_bits(val)
        }
    }
    impl From<Capdhoff> for u8 {
        #[inline(always)]
        fn from(val: Capdhoff) -> u8 {
            Capdhoff::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Capgrp {
        #[doc = "this I3C does not support group address capabilities"]
        B_0X0 = 0,
        #[doc = "this I3C supports group address capabilities (when becoming controller)"]
        B_0X1 = 0x01,
    }
    impl Capgrp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Capgrp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Capgrp {
        #[inline(always)]
        fn from(val: u8) -> Capgrp {
            Capgrp::from_bits(val)
        }
    }
    impl From<Capgrp> for u8 {
        #[inline(always)]
        fn from(val: Capgrp) -> u8 {
            Capgrp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Casupdf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.ASUPDF"]
        B_0X1 = 0x01,
    }
    impl Casupdf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Casupdf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Casupdf {
        #[inline(always)]
        fn from(val: u8) -> Casupdf {
            Casupdf::from_bits(val)
        }
    }
    impl From<Casupdf> for u8 {
        #[inline(always)]
        fn from(val: Casupdf) -> u8 {
            Casupdf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ccrf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.CRF"]
        B_0X1 = 0x01,
    }
    impl Ccrf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ccrf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ccrf {
        #[inline(always)]
        fn from(val: u8) -> Ccrf {
            Ccrf::from_bits(val)
        }
    }
    impl From<Ccrf> for u8 {
        #[inline(always)]
        fn from(val: Ccrf) -> u8 {
            Ccrf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ccrupdf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.CRUPDF"]
        B_0X1 = 0x01,
    }
    impl Ccrupdf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ccrupdf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ccrupdf {
        #[inline(always)]
        fn from(val: u8) -> Ccrupdf {
            Ccrupdf::from_bits(val)
        }
    }
    impl From<Ccrupdf> for u8 {
        #[inline(always)]
        fn from(val: Ccrupdf) -> u8 {
            Ccrupdf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cdaupdf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.DAUPDF"]
        B_0X1 = 0x01,
    }
    impl Cdaupdf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cdaupdf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cdaupdf {
        #[inline(always)]
        fn from(val: u8) -> Cdaupdf {
            Cdaupdf::from_bits(val)
        }
    }
    impl From<Cdaupdf> for u8 {
        #[inline(always)]
        fn from(val: Cdaupdf) -> u8 {
            Cdaupdf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cdeff {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.DEFF"]
        B_0X1 = 0x01,
    }
    impl Cdeff {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cdeff {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cdeff {
        #[inline(always)]
        fn from(val: u8) -> Cdeff {
            Cdeff::from_bits(val)
        }
    }
    impl From<Cdeff> for u8 {
        #[inline(always)]
        fn from(val: Cdeff) -> u8 {
            Cdeff::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cdmaen {
        #[doc = "DMA mode is disabled for C-FIFO"]
        B_0X0 = 0,
        #[doc = "DMA mode is enabled for C-FIFO"]
        B_0X1 = 0x01,
    }
    impl Cdmaen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cdmaen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cdmaen {
        #[inline(always)]
        fn from(val: u8) -> Cdmaen {
            Cdmaen::from_bits(val)
        }
    }
    impl From<Cdmaen> for u8 {
        #[inline(always)]
        fn from(val: Cdmaen) -> u8 {
            Cdmaen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cerrf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.ERRF"]
        B_0X1 = 0x01,
    }
    impl Cerrf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cerrf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cerrf {
        #[inline(always)]
        fn from(val: u8) -> Cerrf {
            Cerrf::from_bits(val)
        }
    }
    impl From<Cerrf> for u8 {
        #[inline(always)]
        fn from(val: Cerrf) -> u8 {
            Cerrf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cfcf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.FCF"]
        B_0X1 = 0x01,
    }
    impl Cfcf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfcf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfcf {
        #[inline(always)]
        fn from(val: u8) -> Cfcf {
            Cfcf::from_bits(val)
        }
    }
    impl From<Cfcf> for u8 {
        #[inline(always)]
        fn from(val: Cfcf) -> u8 {
            Cfcf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cflush {
        #[doc = "no action"]
        B_0X0 = 0,
        #[doc = "flush C-FIFO"]
        B_0X1 = 0x01,
    }
    impl Cflush {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cflush {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cflush {
        #[inline(always)]
        fn from(val: u8) -> Cflush {
            Cflush::from_bits(val)
        }
    }
    impl From<Cflush> for u8 {
        #[inline(always)]
        fn from(val: Cflush) -> u8 {
            Cflush::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cgetf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.GETF"]
        B_0X1 = 0x01,
    }
    impl Cgetf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cgetf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cgetf {
        #[inline(always)]
        fn from(val: u8) -> Cgetf {
            Cgetf::from_bits(val)
        }
    }
    impl From<Cgetf> for u8 {
        #[inline(always)]
        fn from(val: Cgetf) -> u8 {
            Cgetf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cgrpf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.GRPF"]
        B_0X1 = 0x01,
    }
    impl Cgrpf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cgrpf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cgrpf {
        #[inline(always)]
        fn from(val: u8) -> Cgrpf {
            Cgrpf::from_bits(val)
        }
    }
    impl From<Cgrpf> for u8 {
        #[inline(always)]
        fn from(val: Cgrpf) -> u8 {
            Cgrpf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Chjf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.HJF"]
        B_0X1 = 0x01,
    }
    impl Chjf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Chjf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Chjf {
        #[inline(always)]
        fn from(val: u8) -> Chjf {
            Chjf::from_bits(val)
        }
    }
    impl From<Chjf> for u8 {
        #[inline(always)]
        fn from(val: Chjf) -> u8 {
            Chjf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cibiendf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.IBIENDF"]
        B_0X1 = 0x01,
    }
    impl Cibiendf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cibiendf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cibiendf {
        #[inline(always)]
        fn from(val: u8) -> Cibiendf {
            Cibiendf::from_bits(val)
        }
    }
    impl From<Cibiendf> for u8 {
        #[inline(always)]
        fn from(val: Cibiendf) -> u8 {
            Cibiendf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cibif {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.IBIF"]
        B_0X1 = 0x01,
    }
    impl Cibif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cibif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cibif {
        #[inline(always)]
        fn from(val: u8) -> Cibif {
            Cibif::from_bits(val)
        }
    }
    impl From<Cibif> for u8 {
        #[inline(always)]
        fn from(val: Cibif) -> u8 {
            Cibif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cintupdf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.CINTUPDF"]
        B_0X1 = 0x01,
    }
    impl Cintupdf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cintupdf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cintupdf {
        #[inline(always)]
        fn from(val: u8) -> Cintupdf {
            Cintupdf::from_bits(val)
        }
    }
    impl From<Cintupdf> for u8 {
        #[inline(always)]
        fn from(val: Cintupdf) -> u8 {
            Cintupdf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cmrlupdf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.MRLUPDF"]
        B_0X1 = 0x01,
    }
    impl Cmrlupdf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cmrlupdf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cmrlupdf {
        #[inline(always)]
        fn from(val: u8) -> Cmrlupdf {
            Cmrlupdf::from_bits(val)
        }
    }
    impl From<Cmrlupdf> for u8 {
        #[inline(always)]
        fn from(val: Cmrlupdf) -> u8 {
            Cmrlupdf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cmwlupdf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.MWLUPDF"]
        B_0X1 = 0x01,
    }
    impl Cmwlupdf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cmwlupdf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cmwlupdf {
        #[inline(always)]
        fn from(val: u8) -> Cmwlupdf {
            Cmwlupdf::from_bits(val)
        }
    }
    impl From<Cmwlupdf> for u8 {
        #[inline(always)]
        fn from(val: Cmwlupdf) -> u8 {
            Cmwlupdf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Coderr {
        #[doc = "CE0 error (transaction after sending CCC):"]
        B_0X0 = 0,
        #[doc = "CE1 error (monitoring error):"]
        B_0X1 = 0x01,
        #[doc = "CE2 error (no response to broadcast address):"]
        B_0X2 = 0x02,
        #[doc = "CE3 error (failed controller-role hand-off):"]
        B_0X3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "TE0 error (invalid broadcast address 7’hE+W):"]
        B_0X8 = 0x08,
        #[doc = "TE1 error (CCC code):"]
        B_0X9 = 0x09,
        #[doc = "TE2 error (write data):"]
        B_0XA = 0x0a,
        #[doc = "TE3 error (assigned address during dynamic address arbitration):"]
        B_0XB = 0x0b,
        #[doc = "TE4 error (7’hE+R missing after Sr during dynamic address arbitration):"]
        B_0XC = 0x0c,
        #[doc = "TE5 error (transaction after detecting CCC):"]
        B_0XD = 0x0d,
        #[doc = "TE6 error (monitoring error):"]
        B_0XE = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Coderr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Coderr {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Coderr {
        #[inline(always)]
        fn from(val: u8) -> Coderr {
            Coderr::from_bits(val)
        }
    }
    impl From<Coderr> for u8 {
        #[inline(always)]
        fn from(val: Coderr) -> u8 {
            Coderr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Covr {
        #[doc = "no detected error"]
        B_0X0 = 0,
        #[doc = "controller detected either:"]
        B_0X1 = 0x01,
    }
    impl Covr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Covr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Covr {
        #[inline(always)]
        fn from(val: u8) -> Covr {
            Covr::from_bits(val)
        }
    }
    impl From<Covr> for u8 {
        #[inline(always)]
        fn from(val: Covr) -> u8 {
            Covr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Crinit {
        #[doc = "target role"]
        B_0X0 = 0,
        #[doc = "controller role"]
        B_0X1 = 0x01,
    }
    impl Crinit {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Crinit {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Crinit {
        #[inline(always)]
        fn from(val: u8) -> Crinit {
            Crinit::from_bits(val)
        }
    }
    impl From<Crinit> for u8 {
        #[inline(always)]
        fn from(val: Crinit) -> u8 {
            Crinit::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Crstf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.RSTF"]
        B_0X1 = 0x01,
    }
    impl Crstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Crstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Crstf {
        #[inline(always)]
        fn from(val: u8) -> Crstf {
            Crstf::from_bits(val)
        }
    }
    impl From<Crstf> for u8 {
        #[inline(always)]
        fn from(val: Crstf) -> u8 {
            Crstf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Crxtgtendf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.RXTGTENDF"]
        B_0X1 = 0x01,
    }
    impl Crxtgtendf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Crxtgtendf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Crxtgtendf {
        #[inline(always)]
        fn from(val: u8) -> Crxtgtendf {
            Crxtgtendf::from_bits(val)
        }
    }
    impl From<Crxtgtendf> for u8 {
        #[inline(always)]
        fn from(val: Crxtgtendf) -> u8 {
            Crxtgtendf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cstaf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.STAF"]
        B_0X1 = 0x01,
    }
    impl Cstaf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cstaf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cstaf {
        #[inline(always)]
        fn from(val: u8) -> Cstaf {
            Cstaf::from_bits(val)
        }
    }
    impl From<Cstaf> for u8 {
        #[inline(always)]
        fn from(val: Cstaf) -> u8 {
            Cstaf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cwkpf {
        #[doc = "no effect"]
        B_0X0 = 0,
        #[doc = "clear EVR.WKPF"]
        B_0X1 = 0x01,
    }
    impl Cwkpf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cwkpf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cwkpf {
        #[inline(always)]
        fn from(val: u8) -> Cwkpf {
            Cwkpf::from_bits(val)
        }
    }
    impl From<Cwkpf> for u8 {
        #[inline(always)]
        fn from(val: Cwkpf) -> u8 {
            Cwkpf::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcnt(pub u16);
    impl Dcnt {
        #[doc = "no data to transfer (allowed for write message and only for GET CCC read commands. Mandated value when emitting ENTDAA)"]
        pub const B_0X0: Self = Self(0);
        #[doc = "1 byte"]
        pub const B_0X1: Self = Self(0x01);
        #[doc = "2 bytes"]
        pub const B_0X2: Self = Self(0x02);
        #[doc = "64 Kbytes - 1 byte"]
        pub const B_0XFFFF: Self = Self(0xffff);
    }
    impl Dcnt {
        pub const fn from_bits(val: u16) -> Dcnt {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl From<u16> for Dcnt {
        #[inline(always)]
        fn from(val: u16) -> Dcnt {
            Dcnt::from_bits(val)
        }
    }
    impl From<Dcnt> for u16 {
        #[inline(always)]
        fn from(val: Dcnt) -> u16 {
            Dcnt::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcr(pub u8);
    impl Dcr {
        #[doc = "generic device (for v1.0 devices)"]
        pub const B_0X0: Self = Self(0);
    }
    impl Dcr {
        pub const fn from_bits(val: u8) -> Dcr {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl From<u8> for Dcr {
        #[inline(always)]
        fn from(val: u8) -> Dcr {
            Dcr::from_bits(val)
        }
    }
    impl From<Dcr> for u8 {
        #[inline(always)]
        fn from(val: Dcr) -> u8 {
            Dcr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Derr {
        #[doc = "no detected error"]
        B_0X0 = 0,
        #[doc = "controller detected a data error during the controller-role hand-off procedure (GETACCCR CCC, formerly known as GETACCMST) when the received target address or/and the parity bit do no match. Active controller keeps controller-role."]
        B_0X1 = 0x01,
    }
    impl Derr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Derr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Derr {
        #[inline(always)]
        fn from(val: u8) -> Derr {
            Derr::from_bits(val)
        }
    }
    impl From<Derr> for u8 {
        #[inline(always)]
        fn from(val: Derr) -> u8 {
            Derr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr1Crack {
        #[doc = "a controller-role request from target x is to be NACKed"]
        B_0X0 = 0,
        #[doc = "a controller-role request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
        B_0X1 = 0x01,
    }
    impl Devr1Crack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr1Crack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr1Crack {
        #[inline(always)]
        fn from(val: u8) -> Devr1Crack {
            Devr1Crack::from_bits(val)
        }
    }
    impl From<Devr1Crack> for u8 {
        #[inline(always)]
        fn from(val: Devr1Crack) -> u8 {
            Devr1Crack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr1Dis {
        #[doc = "write to DEVRx.DA\\[7:0\\]
and to DEVRx.IBIDEN is allowed"]
        B_0X0 = 0,
        #[doc = "write DEVRx.DA\\[7:0\\]
and to DEVRx.IBIDEN is disabled/locked"]
        B_0X1 = 0x01,
    }
    impl Devr1Dis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr1Dis {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr1Dis {
        #[inline(always)]
        fn from(val: u8) -> Devr1Dis {
            Devr1Dis::from_bits(val)
        }
    }
    impl From<Devr1Dis> for u8 {
        #[inline(always)]
        fn from(val: Devr1Dis) -> u8 {
            Devr1Dis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr1Ibiack {
        #[doc = "an IBI request from target x is to be NACKed"]
        B_0X0 = 0,
        #[doc = "an IBI request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
        B_0X1 = 0x01,
    }
    impl Devr1Ibiack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr1Ibiack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr1Ibiack {
        #[inline(always)]
        fn from(val: u8) -> Devr1Ibiack {
            Devr1Ibiack::from_bits(val)
        }
    }
    impl From<Devr1Ibiack> for u8 {
        #[inline(always)]
        fn from(val: Devr1Ibiack) -> u8 {
            Devr1Ibiack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr1Ibiden {
        #[doc = "no data byte follows the acknowledged IBI from target x"]
        B_0X0 = 0,
        #[doc = "the mandatory data byte MDB\\[7:0\\]
follows the acknowledged IBI from target x"]
        B_0X1 = 0x01,
    }
    impl Devr1Ibiden {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr1Ibiden {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr1Ibiden {
        #[inline(always)]
        fn from(val: u8) -> Devr1Ibiden {
            Devr1Ibiden::from_bits(val)
        }
    }
    impl From<Devr1Ibiden> for u8 {
        #[inline(always)]
        fn from(val: Devr1Ibiden) -> u8 {
            Devr1Ibiden::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr1Susp {
        #[doc = "I3C transfer is not stopped and C-FIFO is not flushed"]
        B_0X0 = 0,
        #[doc = "I3C transfer is stopped and C-FIFO is flushed on a received IBI request from target x"]
        B_0X1 = 0x01,
    }
    impl Devr1Susp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr1Susp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr1Susp {
        #[inline(always)]
        fn from(val: u8) -> Devr1Susp {
            Devr1Susp::from_bits(val)
        }
    }
    impl From<Devr1Susp> for u8 {
        #[inline(always)]
        fn from(val: Devr1Susp) -> u8 {
            Devr1Susp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr2Crack {
        #[doc = "a controller-role request from target x is to be NACKed"]
        B_0X0 = 0,
        #[doc = "a controller-role request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
        B_0X1 = 0x01,
    }
    impl Devr2Crack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr2Crack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr2Crack {
        #[inline(always)]
        fn from(val: u8) -> Devr2Crack {
            Devr2Crack::from_bits(val)
        }
    }
    impl From<Devr2Crack> for u8 {
        #[inline(always)]
        fn from(val: Devr2Crack) -> u8 {
            Devr2Crack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr2Dis {
        #[doc = "write to DEVRx.DA\\[7:0\\]
and to DEVRx.IBIDEN is allowed"]
        B_0X0 = 0,
        #[doc = "write DEVRx.DA\\[7:0\\]
and to DEVRx.IBIDEN is disabled/locked"]
        B_0X1 = 0x01,
    }
    impl Devr2Dis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr2Dis {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr2Dis {
        #[inline(always)]
        fn from(val: u8) -> Devr2Dis {
            Devr2Dis::from_bits(val)
        }
    }
    impl From<Devr2Dis> for u8 {
        #[inline(always)]
        fn from(val: Devr2Dis) -> u8 {
            Devr2Dis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr2Ibiack {
        #[doc = "an IBI request from target x is to be NACKed"]
        B_0X0 = 0,
        #[doc = "an IBI request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
        B_0X1 = 0x01,
    }
    impl Devr2Ibiack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr2Ibiack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr2Ibiack {
        #[inline(always)]
        fn from(val: u8) -> Devr2Ibiack {
            Devr2Ibiack::from_bits(val)
        }
    }
    impl From<Devr2Ibiack> for u8 {
        #[inline(always)]
        fn from(val: Devr2Ibiack) -> u8 {
            Devr2Ibiack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr2Ibiden {
        #[doc = "no data byte follows the acknowledged IBI from target x"]
        B_0X0 = 0,
        #[doc = "the mandatory data byte MDB\\[7:0\\]
follows the acknowledged IBI from target x"]
        B_0X1 = 0x01,
    }
    impl Devr2Ibiden {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr2Ibiden {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr2Ibiden {
        #[inline(always)]
        fn from(val: u8) -> Devr2Ibiden {
            Devr2Ibiden::from_bits(val)
        }
    }
    impl From<Devr2Ibiden> for u8 {
        #[inline(always)]
        fn from(val: Devr2Ibiden) -> u8 {
            Devr2Ibiden::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr2Susp {
        #[doc = "I3C transfer is not stopped and C-FIFO is not flushed"]
        B_0X0 = 0,
        #[doc = "I3C transfer is stopped and C-FIFO is flushed on a received IBI request from target x"]
        B_0X1 = 0x01,
    }
    impl Devr2Susp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr2Susp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr2Susp {
        #[inline(always)]
        fn from(val: u8) -> Devr2Susp {
            Devr2Susp::from_bits(val)
        }
    }
    impl From<Devr2Susp> for u8 {
        #[inline(always)]
        fn from(val: Devr2Susp) -> u8 {
            Devr2Susp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr3Crack {
        #[doc = "a controller-role request from target x is to be NACKed"]
        B_0X0 = 0,
        #[doc = "a controller-role request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
        B_0X1 = 0x01,
    }
    impl Devr3Crack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr3Crack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr3Crack {
        #[inline(always)]
        fn from(val: u8) -> Devr3Crack {
            Devr3Crack::from_bits(val)
        }
    }
    impl From<Devr3Crack> for u8 {
        #[inline(always)]
        fn from(val: Devr3Crack) -> u8 {
            Devr3Crack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr3Dis {
        #[doc = "write to DEVRx.DA\\[7:0\\]
and to DEVRx.IBIDEN is allowed"]
        B_0X0 = 0,
        #[doc = "write DEVRx.DA\\[7:0\\]
and to DEVRx.IBIDEN is disabled/locked"]
        B_0X1 = 0x01,
    }
    impl Devr3Dis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr3Dis {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr3Dis {
        #[inline(always)]
        fn from(val: u8) -> Devr3Dis {
            Devr3Dis::from_bits(val)
        }
    }
    impl From<Devr3Dis> for u8 {
        #[inline(always)]
        fn from(val: Devr3Dis) -> u8 {
            Devr3Dis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr3Ibiack {
        #[doc = "an IBI request from target x is to be NACKed"]
        B_0X0 = 0,
        #[doc = "an IBI request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
        B_0X1 = 0x01,
    }
    impl Devr3Ibiack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr3Ibiack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr3Ibiack {
        #[inline(always)]
        fn from(val: u8) -> Devr3Ibiack {
            Devr3Ibiack::from_bits(val)
        }
    }
    impl From<Devr3Ibiack> for u8 {
        #[inline(always)]
        fn from(val: Devr3Ibiack) -> u8 {
            Devr3Ibiack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr3Ibiden {
        #[doc = "no data byte follows the acknowledged IBI from target x"]
        B_0X0 = 0,
        #[doc = "the mandatory data byte MDB\\[7:0\\]
follows the acknowledged IBI from target x"]
        B_0X1 = 0x01,
    }
    impl Devr3Ibiden {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr3Ibiden {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr3Ibiden {
        #[inline(always)]
        fn from(val: u8) -> Devr3Ibiden {
            Devr3Ibiden::from_bits(val)
        }
    }
    impl From<Devr3Ibiden> for u8 {
        #[inline(always)]
        fn from(val: Devr3Ibiden) -> u8 {
            Devr3Ibiden::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr3Susp {
        #[doc = "I3C transfer is not stopped and C-FIFO is not flushed"]
        B_0X0 = 0,
        #[doc = "I3C transfer is stopped and C-FIFO is flushed on a received IBI request from target x"]
        B_0X1 = 0x01,
    }
    impl Devr3Susp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr3Susp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr3Susp {
        #[inline(always)]
        fn from(val: u8) -> Devr3Susp {
            Devr3Susp::from_bits(val)
        }
    }
    impl From<Devr3Susp> for u8 {
        #[inline(always)]
        fn from(val: Devr3Susp) -> u8 {
            Devr3Susp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr4Crack {
        #[doc = "a controller-role request from target x is to be NACKed"]
        B_0X0 = 0,
        #[doc = "a controller-role request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
        B_0X1 = 0x01,
    }
    impl Devr4Crack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr4Crack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr4Crack {
        #[inline(always)]
        fn from(val: u8) -> Devr4Crack {
            Devr4Crack::from_bits(val)
        }
    }
    impl From<Devr4Crack> for u8 {
        #[inline(always)]
        fn from(val: Devr4Crack) -> u8 {
            Devr4Crack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr4Dis {
        #[doc = "write to DEVRx.DA\\[7:0\\]
and to DEVRx.IBIDEN is allowed"]
        B_0X0 = 0,
        #[doc = "write DEVRx.DA\\[7:0\\]
and to DEVRx.IBIDEN is disabled/locked"]
        B_0X1 = 0x01,
    }
    impl Devr4Dis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr4Dis {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr4Dis {
        #[inline(always)]
        fn from(val: u8) -> Devr4Dis {
            Devr4Dis::from_bits(val)
        }
    }
    impl From<Devr4Dis> for u8 {
        #[inline(always)]
        fn from(val: Devr4Dis) -> u8 {
            Devr4Dis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr4Ibiack {
        #[doc = "an IBI request from target x is to be NACKed"]
        B_0X0 = 0,
        #[doc = "an IBI request (with 7-bit dynamic address DA\\[6:0\\]) from target x is to be ACKed"]
        B_0X1 = 0x01,
    }
    impl Devr4Ibiack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr4Ibiack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr4Ibiack {
        #[inline(always)]
        fn from(val: u8) -> Devr4Ibiack {
            Devr4Ibiack::from_bits(val)
        }
    }
    impl From<Devr4Ibiack> for u8 {
        #[inline(always)]
        fn from(val: Devr4Ibiack) -> u8 {
            Devr4Ibiack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr4Ibiden {
        #[doc = "no data byte follows the acknowledged IBI from target x"]
        B_0X0 = 0,
        #[doc = "the mandatory data byte MDB\\[7:0\\]
follows the acknowledged IBI from target x"]
        B_0X1 = 0x01,
    }
    impl Devr4Ibiden {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr4Ibiden {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr4Ibiden {
        #[inline(always)]
        fn from(val: u8) -> Devr4Ibiden {
            Devr4Ibiden::from_bits(val)
        }
    }
    impl From<Devr4Ibiden> for u8 {
        #[inline(always)]
        fn from(val: Devr4Ibiden) -> u8 {
            Devr4Ibiden::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Devr4Susp {
        #[doc = "I3C transfer is not stopped and C-FIFO is not flushed"]
        B_0X0 = 0,
        #[doc = "I3C transfer is stopped and C-FIFO is flushed on a received IBI request from target x"]
        B_0X1 = 0x01,
    }
    impl Devr4Susp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Devr4Susp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Devr4Susp {
        #[inline(always)]
        fn from(val: u8) -> Devr4Susp {
            Devr4Susp::from_bits(val)
        }
    }
    impl From<Devr4Susp> for u8 {
        #[inline(always)]
        fn from(val: Devr4Susp) -> u8 {
            Devr4Susp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dir {
        #[doc = "write"]
        B_0X0 = 0,
        #[doc = "read"]
        B_0X1 = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dnack {
        #[doc = "no detected error"]
        B_0X0 = 0,
        #[doc = "controller detected that a data byte is not acknowledged by a target, either during:"]
        B_0X1 = 0x01,
    }
    impl Dnack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dnack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dnack {
        #[inline(always)]
        fn from(val: u8) -> Dnack {
            Dnack::from_bits(val)
        }
    }
    impl From<Dnack> for u8 {
        #[inline(always)]
        fn from(val: Dnack) -> u8 {
            Dnack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dovr {
        #[doc = "no detected error"]
        B_0X0 = 0,
        #[doc = "whatever controller or target, hardware detected either:"]
        B_0X1 = 0x01,
    }
    impl Dovr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dovr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dovr {
        #[inline(always)]
        fn from(val: u8) -> Dovr {
            Dovr::from_bits(val)
        }
    }
    impl From<Dovr> for u8 {
        #[inline(always)]
        fn from(val: Dovr) -> u8 {
            Dovr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Exitptrn {
        #[doc = "HDR Exit Pattern is not sent after the message header (MTYPE\\[3:0\\]=0001)"]
        B_0X0 = 0,
        #[doc = "HDR Exit Pattern is sent after the message header (MTYPE\\[3:0\\]=0001) to generate an escalation fault"]
        B_0X1 = 0x01,
    }
    impl Exitptrn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Exitptrn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Exitptrn {
        #[inline(always)]
        fn from(val: u8) -> Exitptrn {
            Exitptrn::from_bits(val)
        }
    }
    impl From<Exitptrn> for u8 {
        #[inline(always)]
        fn from(val: Exitptrn) -> u8 {
            Exitptrn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Fmt {
        #[doc = "format 1 (2 bytes with MaxWr with no defining byte, MaxRd)"]
        B_0X0 = 0,
        #[doc = "format 2: (5 bytes with MaxWr with no defining byte, MaxRd, MaxRdTurn)"]
        B_0X1 = 0x01,
        #[doc = "format 2 (5 bytes with MaxWr with no defining byte, MaxRd, and middle byte of MaxRdTurn)"]
        B_0X2 = 0x02,
        #[doc = "format 2 (5 bytes with MaxWr with no defining byte, MaxRd, MSB of MaxRdTurn)"]
        B_0X3 = 0x03,
    }
    impl Fmt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmt {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmt {
        #[inline(always)]
        fn from(val: u8) -> Fmt {
            Fmt::from_bits(val)
        }
    }
    impl From<Fmt> for u8 {
        #[inline(always)]
        fn from(val: Fmt) -> u8 {
            Fmt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hjack {
        #[doc = "Hot Join request is NACKed"]
        B_0X0 = 0,
        #[doc = "Hot Join request is ACKed"]
        B_0X1 = 0x01,
    }
    impl Hjack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hjack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hjack {
        #[inline(always)]
        fn from(val: u8) -> Hjack {
            Hjack::from_bits(val)
        }
    }
    impl From<Hjack> for u8 {
        #[inline(always)]
        fn from(val: Hjack) -> u8 {
            Hjack::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hksdaen {
        #[doc = "High-Keeper is disabled"]
        B_0X0 = 0,
        #[doc = "High-Keeper is enabled, and the weak pull-up is effective on the T bit, instead of the open-drain class pull-up."]
        B_0X1 = 0x01,
    }
    impl Hksdaen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hksdaen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hksdaen {
        #[inline(always)]
        fn from(val: u8) -> Hksdaen {
            Hksdaen::from_bits(val)
        }
    }
    impl From<Hksdaen> for u8 {
        #[inline(always)]
        fn from(val: Hksdaen) -> u8 {
            Hksdaen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hoffas {
        #[doc = "activity state 0 is the initial activity state of this I3C before and when becoming controller"]
        B_0X0 = 0,
        #[doc = "activity state 1 is the initial activity state of this I3C when becoming controller"]
        B_0X1 = 0x01,
        #[doc = "activity state 2 is the initial activity state of this I3C when becoming controller"]
        B_0X2 = 0x02,
        #[doc = "activity state 3 is the initial activity state of this I3C when becoming controller"]
        B_0X3 = 0x03,
    }
    impl Hoffas {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hoffas {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hoffas {
        #[inline(always)]
        fn from(val: u8) -> Hoffas {
            Hoffas::from_bits(val)
        }
    }
    impl From<Hoffas> for u8 {
        #[inline(always)]
        fn from(val: Hoffas) -> u8 {
            Hoffas::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ibip {
        #[doc = "null payload data size (only allowed when IC3_BCR.BCR2=0)"]
        B_0X0 = 0,
        #[doc = "1 byte (i.e. mandatory data byte MDB\\[7:0\\]"]
        B_0X1 = 0x01,
        #[doc = "2 bytes (including first MDB\\[7:0\\])"]
        B_0X2 = 0x02,
        #[doc = "3 bytes (including first MDB\\[7:0\\])"]
        B_0X3 = 0x03,
        #[doc = "4 bytes (including first MDB\\[7:0\\])"]
        B_0X4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Ibip {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ibip {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ibip {
        #[inline(always)]
        fn from(val: u8) -> Ibip {
            Ibip::from_bits(val)
        }
    }
    impl From<Ibip> for u8 {
        #[inline(always)]
        fn from(val: Ibip) -> u8 {
            Ibip::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mend {
        #[doc = "this message from the controller ends with a Repeated START (Sr)"]
        B_0X0 = 0,
        #[doc = "the message from the controller ends with a STOP (P), being the last message of a frame"]
        B_0X1 = 0x01,
    }
    impl Mend {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mend {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mend {
        #[inline(always)]
        fn from(val: u8) -> Mend {
            Mend::from_bits(val)
        }
    }
    impl From<Mend> for u8 {
        #[inline(always)]
        fn from(val: Mend) -> u8 {
            Mend::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mtype {
        #[doc = "SCL output clock stops running until next control word is executed"]
        B_0X0_WHEN_IS_ACTING_AS_CONTROLLER = 0,
        #[doc = "header message"]
        B_0X1_WHEN_IS_ACTING_AS_CONTROLLER = 0x01,
        #[doc = "private message"]
        B_0X2_WHEN_IS_ACTING_AS_CONTROLLER = 0x02,
        #[doc = "direct message (2nd part of an I3C SDR direct CCC command)"]
        B_0X3_WHEN_IS_ACTING_AS_CONTROLLER = 0x03,
        #[doc = "legacy I2C message"]
        B_0X4_WHEN_IS_ACTING_AS_CONTROLLER = 0x04,
        _RESERVED_5 = 0x05,
        #[doc = "Register is used in alternate usage"]
        B_0X6_WHEN_IS_ACTING_AS_CONTROLLER = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "hot-join request (W)"]
        B_0X8_WHEN_IS_ACTING_AS_TARGET = 0x08,
        #[doc = "controller-role request (W)"]
        B_0X9_WHEN_IS_ACTING_AS_TARGET = 0x09,
        #[doc = "IBI (in-band interrupt) request (R)"]
        B_0XA_WHEN_IS_ACTING_AS_TARGET = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Mtype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mtype {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mtype {
        #[inline(always)]
        fn from(val: u8) -> Mtype {
            Mtype::from_bits(val)
        }
    }
    impl From<Mtype> for u8 {
        #[inline(always)]
        fn from(val: Mtype) -> u8 {
            Mtype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Noarbh {
        #[doc = "An arbitrable header (7’h7E + RnW=0) is emitted after a START and before a legacy I2C message or an I3C SDR private read/write message (default)."]
        B_0X0 = 0,
        #[doc = "No arbitrable header"]
        B_0X1 = 0x01,
    }
    impl Noarbh {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Noarbh {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Noarbh {
        #[inline(always)]
        fn from(val: u8) -> Noarbh {
            Noarbh::from_bits(val)
        }
    }
    impl From<Noarbh> for u8 {
        #[inline(always)]
        fn from(val: Noarbh) -> u8 {
            Noarbh::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Perr {
        #[doc = "no detected error"]
        B_0X0 = 0,
        #[doc = "whatever controller or target, hardware detected a protocol error, as detailed in CODERR\\[3:0\\]"]
        B_0X1 = 0x01,
    }
    impl Perr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Perr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Perr {
        #[inline(always)]
        fn from(val: u8) -> Perr {
            Perr::from_bits(val)
        }
    }
    impl From<Perr> for u8 {
        #[inline(always)]
        fn from(val: Perr) -> u8 {
            Perr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Preload {
        #[doc = "no TX-FIFO preload"]
        B_0X0 = 0,
        #[doc = "TX-FIFO preload"]
        B_0X1 = 0x01,
    }
    impl Preload {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Preload {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Preload {
        #[inline(always)]
        fn from(val: u8) -> Preload {
            Preload::from_bits(val)
        }
    }
    impl From<Preload> for u8 {
        #[inline(always)]
        fn from(val: Preload) -> u8 {
            Preload::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rmode {
        #[doc = "S-FIFO is disabled, and the"]
        B_0X0 = 0,
        #[doc = "S-FIFO is enabled."]
        B_0X1 = 0x01,
    }
    impl Rmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rmode {
        #[inline(always)]
        fn from(val: u8) -> Rmode {
            Rmode::from_bits(val)
        }
    }
    impl From<Rmode> for u8 {
        #[inline(always)]
        fn from(val: Rmode) -> u8 {
            Rmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rnw {
        #[doc = "write message"]
        B_0X0 = 0,
        #[doc = "read message"]
        B_0X1 = 0x01,
    }
    impl Rnw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rnw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rnw {
        #[inline(always)]
        fn from(val: u8) -> Rnw {
            Rnw::from_bits(val)
        }
    }
    impl From<Rnw> for u8 {
        #[inline(always)]
        fn from(val: Rnw) -> u8 {
            Rnw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rstact {
        #[doc = "no reset action"]
        B_0X0 = 0,
        #[doc = "first level of reset: the application software should either:"]
        B_0X1 = 0x01,
        #[doc = "second level of reset: the application software should issue a warm reset, also known as"]
        B_0X2 = 0x02,
        #[doc = "no reset action"]
        B_0X3 = 0x03,
    }
    impl Rstact {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rstact {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rstact {
        #[inline(always)]
        fn from(val: u8) -> Rstact {
            Rstact::from_bits(val)
        }
    }
    impl From<Rstact> for u8 {
        #[inline(always)]
        fn from(val: Rstact) -> u8 {
            Rstact::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rstptrn {
        #[doc = "standard STOP emitted at the end of a frame"]
        B_0X0 = 0,
        #[doc = "HDR reset pattern is inserted before the STOP of any emitted frame that includes a RSTACT CCC command"]
        B_0X1 = 0x01,
    }
    impl Rstptrn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rstptrn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rstptrn {
        #[inline(always)]
        fn from(val: u8) -> Rstptrn {
            Rstptrn::from_bits(val)
        }
    }
    impl From<Rstptrn> for u8 {
        #[inline(always)]
        fn from(val: Rstptrn) -> u8 {
            Rstptrn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rxdmaen {
        #[doc = "DMA mode is disabled for RX-FIFO"]
        B_0X0 = 0,
        #[doc = "DMA mode is enabled for RX-FIFO"]
        B_0X1 = 0x01,
    }
    impl Rxdmaen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rxdmaen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rxdmaen {
        #[inline(always)]
        fn from(val: u8) -> Rxdmaen {
            Rxdmaen::from_bits(val)
        }
    }
    impl From<Rxdmaen> for u8 {
        #[inline(always)]
        fn from(val: Rxdmaen) -> u8 {
            Rxdmaen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rxflush {
        #[doc = "no action"]
        B_0X0 = 0,
        #[doc = "flush RX-FIFO"]
        B_0X1 = 0x01,
    }
    impl Rxflush {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rxflush {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rxflush {
        #[inline(always)]
        fn from(val: u8) -> Rxflush {
            Rxflush::from_bits(val)
        }
    }
    impl From<Rxflush> for u8 {
        #[inline(always)]
        fn from(val: Rxflush) -> u8 {
            Rxflush::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rxthres {
        #[doc = "1-byte threshold"]
        B_0X0 = 0,
        #[doc = "4-byte threshold"]
        B_0X1 = 0x01,
    }
    impl Rxthres {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rxthres {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rxthres {
        #[inline(always)]
        fn from(val: u8) -> Rxthres {
            Rxthres::from_bits(val)
        }
    }
    impl From<Rxthres> for u8 {
        #[inline(always)]
        fn from(val: Rxthres) -> u8 {
            Rxthres::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SdaHd {
        #[doc = "SDA hold time = 0,5 x tI3CCLK"]
        B_0X0 = 0,
        #[doc = "SDA hold time = 1,5 x tI3CCLK"]
        B_0X1 = 0x01,
    }
    impl SdaHd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SdaHd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SdaHd {
        #[inline(always)]
        fn from(val: u8) -> SdaHd {
            SdaHd::from_bits(val)
        }
    }
    impl From<SdaHd> for u8 {
        #[inline(always)]
        fn from(val: SdaHd) -> u8 {
            SdaHd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Sdmaen {
        #[doc = "DMA mode is disabled for S-FIFO"]
        B_0X0 = 0,
        #[doc = "DMA mode is enabled for S-FIFO"]
        B_0X1 = 0x01,
    }
    impl Sdmaen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdmaen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdmaen {
        #[inline(always)]
        fn from(val: u8) -> Sdmaen {
            Sdmaen::from_bits(val)
        }
    }
    impl From<Sdmaen> for u8 {
        #[inline(always)]
        fn from(val: Sdmaen) -> u8 {
            Sdmaen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Sflush {
        #[doc = "no action"]
        B_0X0 = 0,
        #[doc = "flush S-FIFO"]
        B_0X1 = 0x01,
    }
    impl Sflush {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sflush {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sflush {
        #[inline(always)]
        fn from(val: u8) -> Sflush {
            Sflush::from_bits(val)
        }
    }
    impl From<Sflush> for u8 {
        #[inline(always)]
        fn from(val: Sflush) -> u8 {
            Sflush::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Stall {
        #[doc = "no detected error"]
        B_0X0 = 0,
        #[doc = "target detected that SCL was stable for more than 125 �s during a I3C SDR read"]
        B_0X1 = 0x01,
    }
    impl Stall {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stall {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stall {
        #[inline(always)]
        fn from(val: u8) -> Stall {
            Stall::from_bits(val)
        }
    }
    impl From<Stall> for u8 {
        #[inline(always)]
        fn from(val: Stall) -> u8 {
            Stall::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Stalla {
        #[doc = "no stall"]
        B_0X0 = 0,
        #[doc = "stall enabled"]
        B_0X1 = 0x01,
    }
    impl Stalla {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stalla {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stalla {
        #[inline(always)]
        fn from(val: u8) -> Stalla {
            Stalla::from_bits(val)
        }
    }
    impl From<Stalla> for u8 {
        #[inline(always)]
        fn from(val: Stalla) -> u8 {
            Stalla::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tmode {
        #[doc = "C-FIFO and TX-FIFO are not preloaded before starting to emit a frame transfer."]
        B_0X0 = 0,
        #[doc = "C-FIFO and TX-FIFO are first preloaded (also TX-FIFO if needed (depending on the frame format) before starting to emit a frame transfer."]
        B_0X1 = 0x01,
    }
    impl Tmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tmode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tmode {
        #[inline(always)]
        fn from(val: u8) -> Tmode {
            Tmode::from_bits(val)
        }
    }
    impl From<Tmode> for u8 {
        #[inline(always)]
        fn from(val: Tmode) -> u8 {
            Tmode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tsco {
        #[doc = "tSCO <= 12 ns"]
        B_0X0 = 0,
        #[doc = "tSCO > 12 ns (and refer to the datasheet for more details)"]
        B_0X1 = 0x01,
    }
    impl Tsco {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tsco {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tsco {
        #[inline(always)]
        fn from(val: u8) -> Tsco {
            Tsco::from_bits(val)
        }
    }
    impl From<Tsco> for u8 {
        #[inline(always)]
        fn from(val: Tsco) -> u8 {
            Tsco::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tsfset {
        #[doc = "no action"]
        B_0X0 = 0,
        #[doc = "setting this bit initiates a frame transfer by causing the hardware to assert the flag EVR.CFNFF (C-FIFO not full and a control word is needed)"]
        B_0X1 = 0x01,
    }
    impl Tsfset {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tsfset {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tsfset {
        #[inline(always)]
        fn from(val: u8) -> Tsfset {
            Tsfset::from_bits(val)
        }
    }
    impl From<Tsfset> for u8 {
        #[inline(always)]
        fn from(val: Tsfset) -> u8 {
            Tsfset::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Txdmaen {
        #[doc = "DMA mode is disabled for TX-FIFO"]
        B_0X0 = 0,
        #[doc = "DMA mode is enabled for TX-FIFO"]
        B_0X1 = 0x01,
    }
    impl Txdmaen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Txdmaen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Txdmaen {
        #[inline(always)]
        fn from(val: u8) -> Txdmaen {
            Txdmaen::from_bits(val)
        }
    }
    impl From<Txdmaen> for u8 {
        #[inline(always)]
        fn from(val: Txdmaen) -> u8 {
            Txdmaen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Txflush {
        #[doc = "no action"]
        B_0X0 = 0,
        #[doc = "flush TX-FIFO"]
        B_0X1 = 0x01,
    }
    impl Txflush {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Txflush {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Txflush {
        #[inline(always)]
        fn from(val: u8) -> Txflush {
            Txflush::from_bits(val)
        }
    }
    impl From<Txflush> for u8 {
        #[inline(always)]
        fn from(val: Txflush) -> u8 {
            Txflush::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Txthres {
        #[doc = "1-byte threshold"]
        B_0X0 = 0,
        #[doc = "4-byte threshold"]
        B_0X1 = 0x01,
    }
    impl Txthres {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Txthres {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Txthres {
        #[inline(always)]
        fn from(val: u8) -> Txthres {
            Txthres::from_bits(val)
        }
    }
    impl From<Txthres> for u8 {
        #[inline(always)]
        fn from(val: Txthres) -> u8 {
            Txthres::to_bits(val)
        }
    }
}
