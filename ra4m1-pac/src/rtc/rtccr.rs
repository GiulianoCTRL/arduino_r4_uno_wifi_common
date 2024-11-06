#[doc = "Register `RTCCR%s` reader"]
pub type R = crate::R<RTCCR_SPEC>;
#[doc = "Register `RTCCR%s` writer"]
pub type W = crate::W<RTCCR_SPEC>;
#[doc = "Field `TCCT` reader - Time Capture Control"]
pub type TCCT_R = crate::FieldReader<TCCT_A>;
#[doc = "Time Capture Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCT_A {
    #[doc = "0: No event is detected."]
    _00 = 0,
    #[doc = "1: Rising edge is detected."]
    _01 = 1,
    #[doc = "2: Falling edge is detected."]
    _10 = 2,
    #[doc = "3: Both edges are detected."]
    _11 = 3,
}
impl From<TCCT_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCCT_A {
    type Ux = u8;
}
impl TCCT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCCT_A {
        match self.bits {
            0 => TCCT_A::_00,
            1 => TCCT_A::_01,
            2 => TCCT_A::_10,
            3 => TCCT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "No event is detected."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TCCT_A::_00
    }
    #[doc = "Rising edge is detected."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TCCT_A::_01
    }
    #[doc = "Falling edge is detected."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TCCT_A::_10
    }
    #[doc = "Both edges are detected."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TCCT_A::_11
    }
}
#[doc = "Field `TCCT` writer - Time Capture Control"]
pub type TCCT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, TCCT_A>;
impl<'a, REG, const O: u8> TCCT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event is detected."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TCCT_A::_00)
    }
    #[doc = "Rising edge is detected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TCCT_A::_01)
    }
    #[doc = "Falling edge is detected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TCCT_A::_10)
    }
    #[doc = "Both edges are detected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TCCT_A::_11)
    }
}
#[doc = "Field `TCST` reader - Time Capture Status\n\nThe field is **modified** in some way after a read operation."]
pub type TCST_R = crate::BitReader<TCST_A>;
#[doc = "Time Capture Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCST_A {
    #[doc = "0: No event is detected."]
    _0 = 0,
    #[doc = "1: An event is detected."]
    _1 = 1,
}
impl From<TCST_A> for bool {
    #[inline(always)]
    fn from(variant: TCST_A) -> Self {
        variant as u8 != 0
    }
}
impl TCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCST_A {
        match self.bits {
            false => TCST_A::_0,
            true => TCST_A::_1,
        }
    }
    #[doc = "No event is detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCST_A::_0
    }
    #[doc = "An event is detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCST_A::_1
    }
}
#[doc = "Field `TCST` writer - Time Capture Status"]
pub type TCST_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, TCST_A>;
impl<'a, REG, const O: u8> TCST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No event is detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(TCST_A::_0)
    }
    #[doc = "An event is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(TCST_A::_1)
    }
}
#[doc = "Field `TCNF` reader - Time Capture Noise Filter Control"]
pub type TCNF_R = crate::FieldReader<TCNF_A>;
#[doc = "Time Capture Noise Filter Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCNF_A {
    #[doc = "0: The noise filter is off."]
    _00 = 0,
    #[doc = "1: Setting prohibited"]
    _01 = 1,
    #[doc = "2: The noise filter is on (count source)."]
    _10 = 2,
    #[doc = "3: The noise filter is on (count source by divided by 32)."]
    _11 = 3,
}
impl From<TCNF_A> for u8 {
    #[inline(always)]
    fn from(variant: TCNF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCNF_A {
    type Ux = u8;
}
impl TCNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCNF_A {
        match self.bits {
            0 => TCNF_A::_00,
            1 => TCNF_A::_01,
            2 => TCNF_A::_10,
            3 => TCNF_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "The noise filter is off."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TCNF_A::_00
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TCNF_A::_01
    }
    #[doc = "The noise filter is on (count source)."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TCNF_A::_10
    }
    #[doc = "The noise filter is on (count source by divided by 32)."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TCNF_A::_11
    }
}
#[doc = "Field `TCNF` writer - Time Capture Noise Filter Control"]
pub type TCNF_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, TCNF_A>;
impl<'a, REG, const O: u8> TCNF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The noise filter is off."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(TCNF_A::_00)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(TCNF_A::_01)
    }
    #[doc = "The noise filter is on (count source)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(TCNF_A::_10)
    }
    #[doc = "The noise filter is on (count source by divided by 32)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(TCNF_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Time Capture Control"]
    #[inline(always)]
    pub fn tcct(&self) -> TCCT_R {
        TCCT_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Time Capture Status"]
    #[inline(always)]
    pub fn tcst(&self) -> TCST_R {
        TCST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Time Capture Noise Filter Control"]
    #[inline(always)]
    pub fn tcnf(&self) -> TCNF_R {
        TCNF_R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Time Capture Control"]
    #[inline(always)]
    #[must_use]
    pub fn tcct(&mut self) -> TCCT_W<RTCCR_SPEC, 0> {
        TCCT_W::new(self)
    }
    #[doc = "Bit 2 - Time Capture Status"]
    #[inline(always)]
    #[must_use]
    pub fn tcst(&mut self) -> TCST_W<RTCCR_SPEC, 2> {
        TCST_W::new(self)
    }
    #[doc = "Bits 4:5 - Time Capture Noise Filter Control"]
    #[inline(always)]
    #[must_use]
    pub fn tcnf(&mut self) -> TCNF_W<RTCCR_SPEC, 4> {
        TCNF_W::new(self)
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
#[doc = "Time Capture Control Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCR_SPEC;
impl crate::RegisterSpec for RTCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtccr::R`](R) reader structure"]
impl crate::Readable for RTCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtccr::W`](W) writer structure"]
impl crate::Writable for RTCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x04;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCR%s to value 0"]
impl crate::Resettable for RTCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
