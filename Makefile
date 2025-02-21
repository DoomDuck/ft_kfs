.PHONY: all re clean fclean docker
.DEFAULT: all

RM ?= rm
DOCKER_BUILDER_TAG := kfs-builder


all: docker
	docker run -it -v .:/kfs-src $(DOCKER_BUILDER_TAG)

re:
	$(MAKE) fclean
	$(MAKE) all

clean:
	cargo clean
	$(RM) -f kfs/boot.o kfs/libboot.a
	$(RM) -rf isofs/

fclean:
	$(MAKE) clean
	rm -f kfs.iso

docker: Dockerfile
	docker build -t $(DOCKER_BUILDER_TAG) .

run: all
	qemu-system-i386 -cdrom kfs.iso
