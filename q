 PKGBUILD | 4 ++--
 1 file changed, 2 insertions(+), 2 deletions(-)

diff --git a/PKGBUILD b/PKGBUILD
index 6aa0fbc..7293e63 100644
--- a/PKGBUILD
+++ b/PKGBUILD
@@ -3,7 +3,7 @@
 # Contributor: Ivelin Velkov <ivelin dot velkov at gmail dot com>
 
 pkgname=teams-for-linux
-pkgver=1.4.29
+pkgver=1.4.30
 pkgrel=1
 pkgdesc="Unofficial Microsoft Teams client for Linux using Electron."
 arch=("aarch64" "armv7h" "i686" "x86_64")
@@ -15,7 +15,7 @@ source=(
   "${pkgname}-${pkgver}.tar.gz::https://github.com/IsmaelMartinez/${pkgname}/archive/v${pkgver}.tar.gz"
   "${pkgname}.desktop"
 )
-sha256sums=('1ce333afe994e8e67ddb052562287ec958683ba338a9bc25e43f5da6fa52e700'
+sha256sums=('2468e62488d3c2e40c2f6e45a3113ab2734f809d4782c727f63d0b9527291089'
             '4aa7c4aa178ba4e0f97f9ff4a514764a03b332978495d5e7f2dc1ce3f74db615')
 
 build() {




