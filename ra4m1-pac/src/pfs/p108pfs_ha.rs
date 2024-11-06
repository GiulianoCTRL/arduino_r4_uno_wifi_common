#[doc = "Register `P108PFS_HA` reader"]
pub type R = crate::R<P108PFS_HA_SPEC>;
#[doc = "Register `P108PFS_HA` writer"]
pub type W = crate::W<P108PFS_HA_SPEC>;
#[doc = "Field `PODR` reader - Port Output Data"]
pub type PODR_R = crate::BitReader<PODR_A>;
#[doc = "Port Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PODR_A {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<PODR_A> for bool {
    #[inline(always)]
    fn from(variant: PODR_A) -> Self {
        variant as u8 != 0
    }
}
impl PODR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PODR_A {
        match self.bits {
            false => PODR_A::_0,
            true => PODR_A::_1,
        }
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PODR_A::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PODR_A::_1
    }
}
#[doc = "Field `PODR` writer - Port Output Data"]
pub type PODR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PODR_A>;
impl<'a, REG, const O: u8> PODR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PODR_A::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PODR_A::_1)
    }
}
#[doc = "Field `PIDR` reader - Port Input Data"]
pub type PIDR_R = crate::BitReader<PIDR_A>;
#[doc = "Port Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDR_A {
    #[doc = "0: Low input"]
    _0 = 0,
    #[doc = "1: High input"]
    _1 = 1,
}
impl From<PIDR_A> for bool {
    #[inline(always)]
    fn from(variant: PIDR_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIDR_A {
        match self.bits {
            false => PIDR_A::_0,
            true => PIDR_A::_1,
        }
    }
    #[doc = "Low input"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDR_A::_0
    }
    #[doc = "High input"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDR_A::_1
    }
}
#[doc = "Field `PDR` reader - Port Direction"]
pub type PDR_R = crate::BitReader<PDR_A>;
#[doc = "Port Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDR_A {
    #[doc = "0: Input (Functions as an input pin.)"]
    _0 = 0,
    #[doc = "1: Output (Functions as an output pin.)"]
    _1 = 1,
}
impl From<PDR_A> for bool {
    #[inline(always)]
    fn from(variant: PDR_A) -> Self {
        variant as u8 != 0
    }
}
impl PDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDR_A {
        match self.bits {
            false => PDR_A::_0,
            true => PDR_A::_1,
        }
    }
    #[doc = "Input (Functions as an input pin.)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDR_A::_0
    }
    #[doc = "Output (Functions as an output pin.)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDR_A::_1
    }
}
#[doc = "Field `PDR` writer - Port Direction"]
pub type PDR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PDR_A>;
impl<'a, REG, const O: u8> PDR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (Functions as an input pin.)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PDR_A::_0)
    }
    #[doc = "Output (Functions as an output pin.)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PDR_A::_1)
    }
}
#[doc = "Field `PCR` reader - Pull-up Control"]
pub type PCR_R = crate::BitReader<PCR_A>;
#[doc = "Pull-up Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCR_A {
    #[doc = "0: Disables an input pull-up."]
    _0 = 0,
    #[doc = "1: Enables an input pull-up."]
    _1 = 1,
}
impl From<PCR_A> for bool {
    #[inline(always)]
    fn from(variant: PCR_A) -> Self {
        variant as u8 != 0
    }
}
impl PCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCR_A {
        match self.bits {
            false => PCR_A::_0,
            true => PCR_A::_1,
        }
    }
    #[doc = "Disables an input pull-up."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCR_A::_0
    }
    #[doc = "Enables an input pull-up."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCR_A::_1
    }
}
#[doc = "Field `PCR` writer - Pull-up Control"]
pub type PCR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PCR_A>;
impl<'a, REG, const O: u8> PCR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables an input pull-up."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PCR_A::_0)
    }
    #[doc = "Enables an input pull-up."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PCR_A::_1)
    }
}
#[doc = "Field `NCODR` reader - N-Channel Open Drain Control"]
pub type NCODR_R = crate::BitReader<NCODR_A>;
#[doc = "N-Channel Open Drain Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCODR_A {
    #[doc = "0: CMOS output"]
    _0 = 0,
    #[doc = "1: NMOS open-drain output"]
    _1 = 1,
}
impl From<NCODR_A> for bool {
    #[inline(always)]
    fn from(variant: NCODR_A) -> Self {
        variant as u8 != 0
    }
}
impl NCODR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NCODR_A {
        match self.bits {
            false => NCODR_A::_0,
            true => NCODR_A::_1,
        }
    }
    #[doc = "CMOS output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NCODR_A::_0
    }
    #[doc = "NMOS open-drain output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NCODR_A::_1
    }
}
#[doc = "Field `NCODR` writer - N-Channel Open Drain Control"]
pub type NCODR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NCODR_A>;
impl<'a, REG, const O: u8> NCODR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMOS output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(NCODR_A::_0)
    }
    #[doc = "NMOS open-drain output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NCODR_A::_1)
    }
}
#[doc = "Field `DSCR` reader - Port Drive Capability"]
pub type DSCR_R = crate::BitReader<DSCR_A>;
#[doc = "Port Drive Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSCR_A {
    #[doc = "0: Low drive"]
    _0 = 0,
    #[doc = "1: Middle drive."]
    _1 = 1,
}
impl From<DSCR_A> for bool {
    #[inline(always)]
    fn from(variant: DSCR_A) -> Self {
        variant as u8 != 0
    }
}
impl DSCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSCR_A {
        match self.bits {
            false => DSCR_A::_0,
            true => DSCR_A::_1,
        }
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSCR_A::_0
    }
    #[doc = "Middle drive."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSCR_A::_1
    }
}
#[doc = "Field `DSCR` writer - Port Drive Capability"]
pub type DSCR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DSCR_A>;
impl<'a, REG, const O: u8> DSCR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DSCR_A::_0)
    }
    #[doc = "Middle drive."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DSCR_A::_1)
    }
}
#[doc = "Field `EOR` reader - Event on Rising"]
pub type EOR_R = crate::BitReader<EOR_A>;
#[doc = "Event on Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOR_A {
    #[doc = "0: No effected"]
    _0 = 0,
    #[doc = "1: Detect rising edge"]
    _1 = 1,
}
impl From<EOR_A> for bool {
    #[inline(always)]
    fn from(variant: EOR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOR_A {
        match self.bits {
            false => EOR_A::_0,
            true => EOR_A::_1,
        }
    }
    #[doc = "No effected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOR_A::_0
    }
    #[doc = "Detect rising edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOR_A::_1
    }
}
#[doc = "Field `EOR` writer - Event on Rising"]
pub type EOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EOR_A>;
impl<'a, REG, const O: u8> EOR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EOR_A::_0)
    }
    #[doc = "Detect rising edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EOR_A::_1)
    }
}
#[doc = "Field `EOF` reader - Event on Failing"]
pub type EOF_R = crate::BitReader<EOF_A>;
#[doc = "Event on Failing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOF_A {
    #[doc = "0: No effected"]
    _0 = 0,
    #[doc = "1: Detect failing edge"]
    _1 = 1,
}
impl From<EOF_A> for bool {
    #[inline(always)]
    fn from(variant: EOF_A) -> Self {
        variant as u8 != 0
    }
}
impl EOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOF_A {
        match self.bits {
            false => EOF_A::_0,
            true => EOF_A::_1,
        }
    }
    #[doc = "No effected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOF_A::_0
    }
    #[doc = "Detect failing edge"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOF_A::_1
    }
}
#[doc = "Field `EOF` writer - Event on Failing"]
pub type EOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EOF_A>;
impl<'a, REG, const O: u8> EOF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EOF_A::_0)
    }
    #[doc = "Detect failing edge"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EOF_A::_1)
    }
}
#[doc = "Field `ISEL` reader - IRQ input enable"]
pub type ISEL_R = crate::BitReader<ISEL_A>;
#[doc = "IRQ input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISEL_A {
    #[doc = "0: Not used as IRQn input pin"]
    _0 = 0,
    #[doc = "1: Used as IRQn input pin"]
    _1 = 1,
}
impl From<ISEL_A> for bool {
    #[inline(always)]
    fn from(variant: ISEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ISEL_A {
        match self.bits {
            false => ISEL_A::_0,
            true => ISEL_A::_1,
        }
    }
    #[doc = "Not used as IRQn input pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISEL_A::_0
    }
    #[doc = "Used as IRQn input pin"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISEL_A::_1
    }
}
#[doc = "Field `ISEL` writer - IRQ input enable"]
pub type ISEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ISEL_A>;
impl<'a, REG, const O: u8> ISEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not used as IRQn input pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ISEL_A::_0)
    }
    #[doc = "Used as IRQn input pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ISEL_A::_1)
    }
}
#[doc = "Field `ASEL` reader - Analog Input enable"]
pub type ASEL_R = crate::BitReader<ASEL_A>;
#[doc = "Analog Input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASEL_A {
    #[doc = "0: Used other than as analog pin"]
    _0 = 0,
    #[doc = "1: Used as analog pin"]
    _1 = 1,
}
impl From<ASEL_A> for bool {
    #[inline(always)]
    fn from(variant: ASEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ASEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASEL_A {
        match self.bits {
            false => ASEL_A::_0,
            true => ASEL_A::_1,
        }
    }
    #[doc = "Used other than as analog pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ASEL_A::_0
    }
    #[doc = "Used as analog pin"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ASEL_A::_1
    }
}
#[doc = "Field `ASEL` writer - Analog Input enable"]
pub type ASEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ASEL_A>;
impl<'a, REG, const O: u8> ASEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Used other than as analog pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ASEL_A::_0)
    }
    #[doc = "Used as analog pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ASEL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Port Output Data"]
    #[inline(always)]
    pub fn podr(&self) -> PODR_R {
        PODR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Input Data"]
    #[inline(always)]
    pub fn pidr(&self) -> PIDR_R {
        PIDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Direction"]
    #[inline(always)]
    pub fn pdr(&self) -> PDR_R {
        PDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull-up Control"]
    #[inline(always)]
    pub fn pcr(&self) -> PCR_R {
        PCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(&self) -> NCODR_R {
        NCODR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(&self) -> DSCR_R {
        DSCR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Event on Rising"]
    #[inline(always)]
    pub fn eor(&self) -> EOR_R {
        EOR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event on Failing"]
    #[inline(always)]
    pub fn eof(&self) -> EOF_R {
        EOF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - IRQ input enable"]
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Analog Input enable"]
    #[inline(always)]
    pub fn asel(&self) -> ASEL_R {
        ASEL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Output Data"]
    #[inline(always)]
    #[must_use]
    pub fn podr(&mut self) -> PODR_W<P108PFS_HA_SPEC, 0> {
        PODR_W::new(self)
    }
    #[doc = "Bit 2 - Port Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdr(&mut self) -> PDR_W<P108PFS_HA_SPEC, 2> {
        PDR_W::new(self)
    }
    #[doc = "Bit 4 - Pull-up Control"]
    #[inline(always)]
    #[must_use]
    pub fn pcr(&mut self) -> PCR_W<P108PFS_HA_SPEC, 4> {
        PCR_W::new(self)
    }
    #[doc = "Bit 6 - N-Channel Open Drain Control"]
    #[inline(always)]
    #[must_use]
    pub fn ncodr(&mut self) -> NCODR_W<P108PFS_HA_SPEC, 6> {
        NCODR_W::new(self)
    }
    #[doc = "Bit 10 - Port Drive Capability"]
    #[inline(always)]
    #[must_use]
    pub fn dscr(&mut self) -> DSCR_W<P108PFS_HA_SPEC, 10> {
        DSCR_W::new(self)
    }
    #[doc = "Bit 12 - Event on Rising"]
    #[inline(always)]
    #[must_use]
    pub fn eor(&mut self) -> EOR_W<P108PFS_HA_SPEC, 12> {
        EOR_W::new(self)
    }
    #[doc = "Bit 13 - Event on Failing"]
    #[inline(always)]
    #[must_use]
    pub fn eof(&mut self) -> EOF_W<P108PFS_HA_SPEC, 13> {
        EOF_W::new(self)
    }
    #[doc = "Bit 14 - IRQ input enable"]
    #[inline(always)]
    #[must_use]
    pub fn isel(&mut self) -> ISEL_W<P108PFS_HA_SPEC, 14> {
        ISEL_W::new(self)
    }
    #[doc = "Bit 15 - Analog Input enable"]
    #[inline(always)]
    #[must_use]
    pub fn asel(&mut self) -> ASEL_W<P108PFS_HA_SPEC, 15> {
        ASEL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "P108 Pin Function Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`p108pfs_ha::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`p108pfs_ha::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P108PFS_HA_SPEC;
impl crate::RegisterSpec for P108PFS_HA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p108pfs_ha::R`](R) reader structure"]
impl crate::Readable for P108PFS_HA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`p108pfs_ha::W`](W) writer structure"]
impl crate::Writable for P108PFS_HA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P108PFS_HA to value 0x10"]
impl crate::Resettable for P108PFS_HA_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
