#[doc = "Register `SARU%s` reader"]
pub type R = crate::R<SARU_SPEC>;
#[doc = "Register `SARU%s` writer"]
pub type W = crate::W<SARU_SPEC>;
#[doc = "Field `FS` reader - 7-Bit/10-Bit Address Format Selection"]
pub type FS_R = crate::BitReader<FS_A>;
#[doc = "7-Bit/10-Bit Address Format Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FS_A {
    #[doc = "0: The 7-bit address format is selected."]
    _0 = 0,
    #[doc = "1: The 10-bit address format is selected."]
    _1 = 1,
}
impl From<FS_A> for bool {
    #[inline(always)]
    fn from(variant: FS_A) -> Self {
        variant as u8 != 0
    }
}
impl FS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FS_A {
        match self.bits {
            false => FS_A::_0,
            true => FS_A::_1,
        }
    }
    #[doc = "The 7-bit address format is selected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FS_A::_0
    }
    #[doc = "The 10-bit address format is selected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FS_A::_1
    }
}
#[doc = "Field `FS` writer - 7-Bit/10-Bit Address Format Selection"]
pub type FS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FS_A>;
impl<'a, REG, const O: u8> FS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The 7-bit address format is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FS_A::_0)
    }
    #[doc = "The 10-bit address format is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FS_A::_1)
    }
}
#[doc = "Field `SVA8` reader - 10-Bit Address(bit8)"]
pub type SVA8_R = crate::BitReader;
#[doc = "Field `SVA8` writer - 10-Bit Address(bit8)"]
pub type SVA8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SVA9` reader - 10-Bit Address(bit9)"]
pub type SVA9_R = crate::BitReader;
#[doc = "Field `SVA9` writer - 10-Bit Address(bit9)"]
pub type SVA9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 7-Bit/10-Bit Address Format Selection"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 10-Bit Address(bit8)"]
    #[inline(always)]
    pub fn sva8(&self) -> SVA8_R {
        SVA8_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 10-Bit Address(bit9)"]
    #[inline(always)]
    pub fn sva9(&self) -> SVA9_R {
        SVA9_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 7-Bit/10-Bit Address Format Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fs(&mut self) -> FS_W<SARU_SPEC, 0> {
        FS_W::new(self)
    }
    #[doc = "Bit 1 - 10-Bit Address(bit8)"]
    #[inline(always)]
    #[must_use]
    pub fn sva8(&mut self) -> SVA8_W<SARU_SPEC, 1> {
        SVA8_W::new(self)
    }
    #[doc = "Bit 2 - 10-Bit Address(bit9)"]
    #[inline(always)]
    #[must_use]
    pub fn sva9(&mut self) -> SVA9_W<SARU_SPEC, 2> {
        SVA9_W::new(self)
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
#[doc = "Slave Address Register U%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saru::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saru::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SARU_SPEC;
impl crate::RegisterSpec for SARU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`saru::R`](R) reader structure"]
impl crate::Readable for SARU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`saru::W`](W) writer structure"]
impl crate::Writable for SARU_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SARU%s to value 0"]
impl crate::Resettable for SARU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
