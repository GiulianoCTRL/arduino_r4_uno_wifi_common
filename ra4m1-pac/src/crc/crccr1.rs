#[doc = "Register `CRCCR1` reader"]
pub type R = crate::R<CRCCR1_SPEC>;
#[doc = "Register `CRCCR1` writer"]
pub type W = crate::W<CRCCR1_SPEC>;
#[doc = "Field `CRCSWR` reader - Snoop-on-write/read switch bit"]
pub type CRCSWR_R = crate::BitReader<CRCSWR_A>;
#[doc = "Snoop-on-write/read switch bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSWR_A {
    #[doc = "0: Snoop-on-read"]
    _0 = 0,
    #[doc = "1: Snoop-on-write"]
    _1 = 1,
}
impl From<CRCSWR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCSWR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCSWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCSWR_A {
        match self.bits {
            false => CRCSWR_A::_0,
            true => CRCSWR_A::_1,
        }
    }
    #[doc = "Snoop-on-read"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCSWR_A::_0
    }
    #[doc = "Snoop-on-write"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCSWR_A::_1
    }
}
#[doc = "Field `CRCSWR` writer - Snoop-on-write/read switch bit"]
pub type CRCSWR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CRCSWR_A>;
impl<'a, REG, const O: u8> CRCSWR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Snoop-on-read"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSWR_A::_0)
    }
    #[doc = "Snoop-on-write"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSWR_A::_1)
    }
}
#[doc = "Field `CRCSEN` reader - Snoop enable bit"]
pub type CRCSEN_R = crate::BitReader<CRCSEN_A>;
#[doc = "Snoop enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCSEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CRCSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCSEN_A {
        match self.bits {
            false => CRCSEN_A::_0,
            true => CRCSEN_A::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCSEN_A::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCSEN_A::_1
    }
}
#[doc = "Field `CRCSEN` writer - Snoop enable bit"]
pub type CRCSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CRCSEN_A>;
impl<'a, REG, const O: u8> CRCSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCSEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 6 - Snoop-on-write/read switch bit"]
    #[inline(always)]
    pub fn crcswr(&self) -> CRCSWR_R {
        CRCSWR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Snoop enable bit"]
    #[inline(always)]
    pub fn crcsen(&self) -> CRCSEN_R {
        CRCSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Snoop-on-write/read switch bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcswr(&mut self) -> CRCSWR_W<CRCCR1_SPEC, 6> {
        CRCSWR_W::new(self)
    }
    #[doc = "Bit 7 - Snoop enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcsen(&mut self) -> CRCSEN_W<CRCCR1_SPEC, 7> {
        CRCSEN_W::new(self)
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
#[doc = "CRC Control Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crccr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crccr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCCR1_SPEC;
impl crate::RegisterSpec for CRCCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crccr1::R`](R) reader structure"]
impl crate::Readable for CRCCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crccr1::W`](W) writer structure"]
impl crate::Writable for CRCCR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCCR1 to value 0"]
impl crate::Resettable for CRCCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
