#[doc = "Register `SPND` reader"]
pub type R = crate::R<SPND_SPEC>;
#[doc = "Register `SPND` writer"]
pub type W = crate::W<SPND_SPEC>;
#[doc = "Field `SPNDL` reader - SPI Next-Access Delay Setting"]
pub type SPNDL_R = crate::FieldReader<SPNDL_A>;
#[doc = "SPI Next-Access Delay Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPNDL_A {
    #[doc = "0: 1 RSPCK + 2 PCLK"]
    _000 = 0,
    #[doc = "1: 2 RSPCK + 2 PCLK"]
    _001 = 1,
    #[doc = "2: 3 RSPCK + 2 PCLK"]
    _010 = 2,
    #[doc = "3: 4 RSPCK + 2 PCLK"]
    _011 = 3,
    #[doc = "4: 5 RSPCK + 2 PCLK"]
    _100 = 4,
    #[doc = "5: 6 RSPCK + 2 PCLK"]
    _101 = 5,
    #[doc = "6: 7 RSPCK + 2 PCLK"]
    _110 = 6,
    #[doc = "7: 8 RSPCK + 2 PCLK"]
    _111 = 7,
}
impl From<SPNDL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPNDL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPNDL_A {
    type Ux = u8;
}
impl SPNDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPNDL_A {
        match self.bits {
            0 => SPNDL_A::_000,
            1 => SPNDL_A::_001,
            2 => SPNDL_A::_010,
            3 => SPNDL_A::_011,
            4 => SPNDL_A::_100,
            5 => SPNDL_A::_101,
            6 => SPNDL_A::_110,
            7 => SPNDL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "1 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPNDL_A::_000
    }
    #[doc = "2 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPNDL_A::_001
    }
    #[doc = "3 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPNDL_A::_010
    }
    #[doc = "4 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPNDL_A::_011
    }
    #[doc = "5 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPNDL_A::_100
    }
    #[doc = "6 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPNDL_A::_101
    }
    #[doc = "7 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPNDL_A::_110
    }
    #[doc = "8 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPNDL_A::_111
    }
}
#[doc = "Field `SPNDL` writer - SPI Next-Access Delay Setting"]
pub type SPNDL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, SPNDL_A>;
impl<'a, REG, const O: u8> SPNDL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_000)
    }
    #[doc = "2 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_001)
    }
    #[doc = "3 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_010)
    }
    #[doc = "4 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_011)
    }
    #[doc = "5 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_100)
    }
    #[doc = "6 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_101)
    }
    #[doc = "7 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_110)
    }
    #[doc = "8 RSPCK + 2 PCLK"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(SPNDL_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI Next-Access Delay Setting"]
    #[inline(always)]
    pub fn spndl(&self) -> SPNDL_R {
        SPNDL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI Next-Access Delay Setting"]
    #[inline(always)]
    #[must_use]
    pub fn spndl(&mut self) -> SPNDL_W<SPND_SPEC, 0> {
        SPNDL_W::new(self)
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
#[doc = "SPI Next-Access Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spnd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spnd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPND_SPEC;
impl crate::RegisterSpec for SPND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spnd::R`](R) reader structure"]
impl crate::Readable for SPND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spnd::W`](W) writer structure"]
impl crate::Writable for SPND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPND to value 0"]
impl crate::Resettable for SPND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
