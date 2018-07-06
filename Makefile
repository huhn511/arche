dist=dist

build: api www
	cd $(dist) && tar cfJ ../$(dist).tar.xz *

api:
	cargo build --release
	strip -s target/release/arche
	mkdir -p $(dist)/public $(dist)/tmp
	-cp -r target/release/arche templates themes log4rs.yml package.json package-lock.json LICENSE README.md $(dist)/

www:
	cd dashboard && npm run build
	-cp -r dashboard/build $(dist)/dashboard

schema:
	DATABASE_URL="postgres://postgres:@localhost:5432/arche" diesel print-schema > src/orm/postgresql/schema.rs
	DATABASE_URL="mysql://root:test@localhost:3306/arche" diesel print-schema > src/orm/mysql/schema.rs

clean:
	cargo clean
	-rm -r $(dist) $(dist).tar.xz dashboard/build
