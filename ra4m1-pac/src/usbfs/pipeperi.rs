#[doc = "Register `PIPEPERI` reader"]
pub type R = crate::R<PIPEPERI_SPEC>;
#[doc = "Register `PIPEPERI` writer"]
pub type W = crate::W<PIPEPERI_SPEC>;
#[doc = "Field `IITV` reader - Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
pub type IITV_R = crate::FieldReader;
#[doc = "Field `IITV` writer - Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
pub type IITV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `IFIS` reader - Isochronous IN Buffer Flush"]
pub type IFIS_R = crate::BitReader<IFIS_A>;
#[doc = "Isochronous IN Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IFIS_A {
    #[doc = "0: The buffer is not flushed."]
    _0 = 0,
    #[doc = "1: The buffer is flushed."]
    _1 = 1,
}
impl From<IFIS_A> for bool {
    #[inline(always)]
    fn from(variant: IFIS_A) -> Self {
        variant as u8 != 0
    }
}
impl IFIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IFIS_A {
        match self.bits {
            false => IFIS_A::_0,
            true => IFIS_A::_1,
        }
    }
    #[doc = "The buffer is not flushed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IFIS_A::_0
    }
    #[doc = "The buffer is flushed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IFIS_A::_1
    }
}
#[doc = "Field `IFIS` writer - Isochronous IN Buffer Flush"]
pub type IFIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, IFIS_A>;
impl<'a, REG, const O: u8> IFIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The buffer is not flushed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(IFIS_A::_0)
    }
    #[doc = "The buffer is flushed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(IFIS_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
    #[inline(always)]
    pub fn iitv(&self) -> IITV_R {
        IITV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 12 - Isochronous IN Buffer Flush"]
    #[inline(always)]
    pub fn ifis(&self) -> IFIS_R {
        IFIS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Interval Error Detection Interval Specifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
    #[inline(always)]
    #[must_use]
    pub fn iitv(&mut self) -> IITV_W<PIPEPERI_SPEC, 0> {
        IITV_W::new(self)
    }
    #[doc = "Bit 12 - Isochronous IN Buffer Flush"]
    #[inline(always)]
    #[must_use]
    pub fn ifis(&mut self) -> IFIS_W<PIPEPERI_SPEC, 12> {
        IFIS_W::new(self)
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
#[doc = "Pipe Cycle Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipeperi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipeperi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIPEPERI_SPEC;
impl crate::RegisterSpec for PIPEPERI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipeperi::R`](R) reader structure"]
impl crate::Readable for PIPEPERI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pipeperi::W`](W) writer structure"]
impl crate::Writable for PIPEPERI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPEPERI to value 0"]
impl crate::Resettable for PIPEPERI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
