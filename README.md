# Sanca

Using regexes, Sanca tries to recognize software based on their banner or the
data they transmit (e.g. HTTP responses). Moreover, it provides an evidence
for each finding and is also able to give a list of vulnerabilities for the
recognized software.

## Usage

```
$ ./sanca --help
A scanner to discover what technologies are running on a remote host. Focus on providing evidences.

Usage: sanca_software [OPTIONS] --scan-type <SCAN_TYPE>

Options:
  -u, --url <URL>                    The URL where to send an HTTP request
  -i, --ip-hostname <IP_HOSTNAME>    The IP or hostname to connect on
  -p, --port <PORT>                  The port to connect on
  -s, --scan-type <SCAN_TYPE>        The type of scan [possible values: tcp, http, udp]
  -t, --technologies <TECHNOLOGIES>  The technologies to check [possible values: dovecot, exim, mariadb, mysql, openssh, proftpd, pureftpd, os, php, phpmyadmin, wordpress, drupal, typo3, httpd, nginx, openssl, jquery, reactjs, handlebars, lodash, angularjs, gsap, tomcat, bootstrap, angular, plesk, ckeditor, highcharts, yoastseo, revslider, jscomposer, contactform, melis, elementor, elementreadylite, gtranslate, woocommerce, divi, classiceditor, akismet, wpformslite, allinonewpmigration, reallysimplessl, jetpack, litespeedcache, allinoneseo, wordfence, wpmailsmtp, mc4wp, spectra, squirrelmail, phonesystem3cx, prestashop, jira, twisted, twistedweb, symfony, tinymce, jqueryui, layerslider, wpmembers, forminator, horde, knockout, wpsupercache, emailsubscribers, bettersearchreplace, advancedcustomfields, healthcheck, jquerymobile]
  -w, --writer <WRITER>              The writer to use [default: textstdout] [possible values: textstdout, csv, json]
  -a, --user-agent <USER_AGENT>      The user agent [default: Sanca]
  -e, --hide-header                  Hide the header with the URL to the Sanca's website
      --vuln-source <VULN_SOURCE>    The source where download the CVEs to match the findings against. Only the technology and the version are transmitted [possible values: nvd]
      --vuln-cache <VULN_CACHE>      The type of cache to use to store the downloaded vulnerabilities. Can be used only if vuln-source is given [possible values: files]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Building from source

```shell
git clone https://github.com/nalysius/sanca-software
cd sanca-software
cargo build --release
```

> Note: if on OpenBSD the program fails with a SIGSEGV, it's recommended
> to compile with
> cargo build --release --no-default-features --features ring-provider
> The reason is that OpenBSD has strict rules about memory and aws-lc,
> the default provider used by rustls, seems to violate them sometimes.
> The "ring-provider" feature means that the ring provider will be used
> instead of aws-lc.

## Examples

### TCP scan

```
./sanca -s tcp -p 22 -i example.org
./sanca -s tcp -p 22 -i example.org --vuln-source nvd --vuln-cache files
```

Both examples above perform a scan of port 22/tcp, on host example.org. The
second one queries the NVD API to find the CVEs affecting the detected software.
To save time, the vulnerabilities are saved in cache.

### HTTP scan

```
./sanca -s http -u https://example.com/phpmyadmin
./sanca -s http -t wordpress -u https://example.com/blog/index.php --vuln-source nvd --vuln-cache files
```

The first example above performs a HTTP scan at the URL `https://example.com/phpmyadmin`,
and another at `https://example.com/blog/index.php`, where only WordPress will be
checked. The second example also fetches the vulnerabilities from the NVD and
stores them in cache.

