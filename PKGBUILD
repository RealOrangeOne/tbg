# Maintainer: Jake Howard <git+aur@theorangeone.net>
pkgname=tbg
pkgver=0.2.0
pkgrel=0
pkgdesc="Use feh to randomize desktop backgrounds from a directory "
makedepends=('rust' 'cargo')
license=('unknown')
depends=('feh')
url="https://github.com/RealOrangeOne/tbg"
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
source=("$pkgver.tar.gz::https://github.com/RealOrangeOne/tbg/archive/$pkgver.tar.gz")
sha512sums=('SKIP')

package() {
    cd $pkgname-$pkgver
    cargo build --release
    install -D -m755 "target/release/tbg" "$pkgdir/usr/bin/tbg"
}
