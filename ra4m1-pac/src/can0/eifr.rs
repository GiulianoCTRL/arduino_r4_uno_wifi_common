#[doc = "Register `EIFR` reader"]
pub type R = crate::R<EIFR_SPEC>;
#[doc = "Register `EIFR` writer"]
pub type W = crate::W<EIFR_SPEC>;
#[doc = "Field `BEIF` reader - Bus Error Detect Flag"]
pub type BEIF_R = crate::BitReader<BEIF_A>;
#[doc = "Bus Error Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEIF_A {
    #[doc = "0: No bus error detected"]
    _0 = 0,
    #[doc = "1: Bus error detected"]
    _1 = 1,
}
impl From<BEIF_A> for bool {
    #[inline(always)]
    fn from(variant: BEIF_A) -> Self {
        variant as u8 != 0
    }
}
impl BEIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BEIF_A {
        match self.bits {
            false => BEIF_A::_0,
            true => BEIF_A::_1,
        }
    }
    #[doc = "No bus error detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEIF_A::_0
    }
    #[doc = "Bus error detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEIF_A::_1
    }
}
#[doc = "Field `BEIF` writer - Bus Error Detect Flag"]
pub type BEIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BEIF_A>;
impl<'a, REG, const O: u8> BEIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bus error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BEIF_A::_0)
    }
    #[doc = "Bus error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BEIF_A::_1)
    }
}
#[doc = "Field `EWIF` reader - Error-Warning Detect Flag"]
pub type EWIF_R = crate::BitReader<EWIF_A>;
#[doc = "Error-Warning Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIF_A {
    #[doc = "0: No error-warning detected"]
    _0 = 0,
    #[doc = "1: Error-warning detected"]
    _1 = 1,
}
impl From<EWIF_A> for bool {
    #[inline(always)]
    fn from(variant: EWIF_A) -> Self {
        variant as u8 != 0
    }
}
impl EWIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EWIF_A {
        match self.bits {
            false => EWIF_A::_0,
            true => EWIF_A::_1,
        }
    }
    #[doc = "No error-warning detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWIF_A::_0
    }
    #[doc = "Error-warning detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWIF_A::_1
    }
}
#[doc = "Field `EWIF` writer - Error-Warning Detect Flag"]
pub type EWIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EWIF_A>;
impl<'a, REG, const O: u8> EWIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error-warning detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EWIF_A::_0)
    }
    #[doc = "Error-warning detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EWIF_A::_1)
    }
}
#[doc = "Field `EPIF` reader - Error-Passive Detect Flag"]
pub type EPIF_R = crate::BitReader<EPIF_A>;
#[doc = "Error-Passive Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPIF_A {
    #[doc = "0: No error-passive detected"]
    _0 = 0,
    #[doc = "1: Error-passive detected"]
    _1 = 1,
}
impl From<EPIF_A> for bool {
    #[inline(always)]
    fn from(variant: EPIF_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPIF_A {
        match self.bits {
            false => EPIF_A::_0,
            true => EPIF_A::_1,
        }
    }
    #[doc = "No error-passive detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPIF_A::_0
    }
    #[doc = "Error-passive detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPIF_A::_1
    }
}
#[doc = "Field `EPIF` writer - Error-Passive Detect Flag"]
pub type EPIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EPIF_A>;
impl<'a, REG, const O: u8> EPIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error-passive detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(EPIF_A::_0)
    }
    #[doc = "Error-passive detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(EPIF_A::_1)
    }
}
#[doc = "Field `BOEIF` reader - Bus-Off Entry Detect Flag"]
pub type BOEIF_R = crate::BitReader<BOEIF_A>;
#[doc = "Bus-Off Entry Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOEIF_A {
    #[doc = "0: No bus-off entry detected"]
    _0 = 0,
    #[doc = "1: Bus-off entry detected"]
    _1 = 1,
}
impl From<BOEIF_A> for bool {
    #[inline(always)]
    fn from(variant: BOEIF_A) -> Self {
        variant as u8 != 0
    }
}
impl BOEIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BOEIF_A {
        match self.bits {
            false => BOEIF_A::_0,
            true => BOEIF_A::_1,
        }
    }
    #[doc = "No bus-off entry detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOEIF_A::_0
    }
    #[doc = "Bus-off entry detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOEIF_A::_1
    }
}
#[doc = "Field `BOEIF` writer - Bus-Off Entry Detect Flag"]
pub type BOEIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BOEIF_A>;
impl<'a, REG, const O: u8> BOEIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bus-off entry detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BOEIF_A::_0)
    }
    #[doc = "Bus-off entry detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BOEIF_A::_1)
    }
}
#[doc = "Field `BORIF` reader - Bus-Off Recovery Detect Flag"]
pub type BORIF_R = crate::BitReader<BORIF_A>;
#[doc = "Bus-Off Recovery Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BORIF_A {
    #[doc = "0: No bus-off recovery detected"]
    _0 = 0,
    #[doc = "1: Bus-off recovery detected"]
    _1 = 1,
}
impl From<BORIF_A> for bool {
    #[inline(always)]
    fn from(variant: BORIF_A) -> Self {
        variant as u8 != 0
    }
}
impl BORIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BORIF_A {
        match self.bits {
            false => BORIF_A::_0,
            true => BORIF_A::_1,
        }
    }
    #[doc = "No bus-off recovery detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BORIF_A::_0
    }
    #[doc = "Bus-off recovery detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BORIF_A::_1
    }
}
#[doc = "Field `BORIF` writer - Bus-Off Recovery Detect Flag"]
pub type BORIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BORIF_A>;
impl<'a, REG, const O: u8> BORIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bus-off recovery detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BORIF_A::_0)
    }
    #[doc = "Bus-off recovery detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BORIF_A::_1)
    }
}
#[doc = "Field `ORIF` reader - Receive Overrun Detect Flag"]
pub type ORIF_R = crate::BitReader<ORIF_A>;
#[doc = "Receive Overrun Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORIF_A {
    #[doc = "0: No receive overrun detected"]
    _0 = 0,
    #[doc = "1: Receive overrun detected"]
    _1 = 1,
}
impl From<ORIF_A> for bool {
    #[inline(always)]
    fn from(variant: ORIF_A) -> Self {
        variant as u8 != 0
    }
}
impl ORIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ORIF_A {
        match self.bits {
            false => ORIF_A::_0,
            true => ORIF_A::_1,
        }
    }
    #[doc = "No receive overrun detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORIF_A::_0
    }
    #[doc = "Receive overrun detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORIF_A::_1
    }
}
#[doc = "Field `ORIF` writer - Receive Overrun Detect Flag"]
pub type ORIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ORIF_A>;
impl<'a, REG, const O: u8> ORIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No receive overrun detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ORIF_A::_0)
    }
    #[doc = "Receive overrun detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ORIF_A::_1)
    }
}
#[doc = "Field `OLIF` reader - Overload Frame Transmission Detect Flag"]
pub type OLIF_R = crate::BitReader<OLIF_A>;
#[doc = "Overload Frame Transmission Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OLIF_A {
    #[doc = "0: No overload frame transmission detected"]
    _0 = 0,
    #[doc = "1: Overload frame transmission detected"]
    _1 = 1,
}
impl From<OLIF_A> for bool {
    #[inline(always)]
    fn from(variant: OLIF_A) -> Self {
        variant as u8 != 0
    }
}
impl OLIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OLIF_A {
        match self.bits {
            false => OLIF_A::_0,
            true => OLIF_A::_1,
        }
    }
    #[doc = "No overload frame transmission detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OLIF_A::_0
    }
    #[doc = "Overload frame transmission detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OLIF_A::_1
    }
}
#[doc = "Field `OLIF` writer - Overload Frame Transmission Detect Flag"]
pub type OLIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OLIF_A>;
impl<'a, REG, const O: u8> OLIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overload frame transmission detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OLIF_A::_0)
    }
    #[doc = "Overload frame transmission detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OLIF_A::_1)
    }
}
#[doc = "Field `BLIF` reader - Bus Lock Detect Flag"]
pub type BLIF_R = crate::BitReader<BLIF_A>;
#[doc = "Bus Lock Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLIF_A {
    #[doc = "0: No bus lock detected"]
    _0 = 0,
    #[doc = "1: Bus lock detected"]
    _1 = 1,
}
impl From<BLIF_A> for bool {
    #[inline(always)]
    fn from(variant: BLIF_A) -> Self {
        variant as u8 != 0
    }
}
impl BLIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BLIF_A {
        match self.bits {
            false => BLIF_A::_0,
            true => BLIF_A::_1,
        }
    }
    #[doc = "No bus lock detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLIF_A::_0
    }
    #[doc = "Bus lock detected"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLIF_A::_1
    }
}
#[doc = "Field `BLIF` writer - Bus Lock Detect Flag"]
pub type BLIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BLIF_A>;
impl<'a, REG, const O: u8> BLIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bus lock detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(BLIF_A::_0)
    }
    #[doc = "Bus lock detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(BLIF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Bus Error Detect Flag"]
    #[inline(always)]
    pub fn beif(&self) -> BEIF_R {
        BEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error-Warning Detect Flag"]
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error-Passive Detect Flag"]
    #[inline(always)]
    pub fn epif(&self) -> EPIF_R {
        EPIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus-Off Entry Detect Flag"]
    #[inline(always)]
    pub fn boeif(&self) -> BOEIF_R {
        BOEIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus-Off Recovery Detect Flag"]
    #[inline(always)]
    pub fn borif(&self) -> BORIF_R {
        BORIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun Detect Flag"]
    #[inline(always)]
    pub fn orif(&self) -> ORIF_R {
        ORIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overload Frame Transmission Detect Flag"]
    #[inline(always)]
    pub fn olif(&self) -> OLIF_R {
        OLIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Lock Detect Flag"]
    #[inline(always)]
    pub fn blif(&self) -> BLIF_R {
        BLIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Error Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn beif(&mut self) -> BEIF_W<EIFR_SPEC, 0> {
        BEIF_W::new(self)
    }
    #[doc = "Bit 1 - Error-Warning Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ewif(&mut self) -> EWIF_W<EIFR_SPEC, 1> {
        EWIF_W::new(self)
    }
    #[doc = "Bit 2 - Error-Passive Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn epif(&mut self) -> EPIF_W<EIFR_SPEC, 2> {
        EPIF_W::new(self)
    }
    #[doc = "Bit 3 - Bus-Off Entry Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn boeif(&mut self) -> BOEIF_W<EIFR_SPEC, 3> {
        BOEIF_W::new(self)
    }
    #[doc = "Bit 4 - Bus-Off Recovery Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn borif(&mut self) -> BORIF_W<EIFR_SPEC, 4> {
        BORIF_W::new(self)
    }
    #[doc = "Bit 5 - Receive Overrun Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn orif(&mut self) -> ORIF_W<EIFR_SPEC, 5> {
        ORIF_W::new(self)
    }
    #[doc = "Bit 6 - Overload Frame Transmission Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn olif(&mut self) -> OLIF_W<EIFR_SPEC, 6> {
        OLIF_W::new(self)
    }
    #[doc = "Bit 7 - Bus Lock Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn blif(&mut self) -> BLIF_W<EIFR_SPEC, 7> {
        BLIF_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Error Interrupt Factor Judge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EIFR_SPEC;
impl crate::RegisterSpec for EIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eifr::R`](R) reader structure"]
impl crate::Readable for EIFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eifr::W`](W) writer structure"]
impl crate::Writable for EIFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIFR to value 0"]
impl crate::Resettable for EIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
