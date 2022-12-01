/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package com.github.windoze.feathr.piper;

import java.io.*;
import java.net.URL;
import java.util.Map;

public class PiperService implements AutoCloseable {
    private static native long create(String pipelines, String lookups, Map<String, UserDefinedFunction> functions);

    private static native void start(long handle, String address, short port);

    private static native void stop(long handle);

    private static native void destroy(long handle);

    private final long svcHandle;

    static {
        /*
         * Following JNI libs should be included
         * Windows:
         *  feathr_piper_jni_windows_amd64.dll
         * MacOS:
         *  libfeathr_piper_jni_osx_amd64.dylib
         *  libfeathr_piper_jni_osx_aarch64.dylib
         * Linux:
         *  libfeathr_piper_jni_linux_amd64.so
         *  libfeathr_piper_jni_linux_aarch64.so
         */
        loadLibrary("feathr_piper_jni");
    }

    private static String getLibName(String base) {
        String arch = System.getProperty("os.arch").toLowerCase();
        if (arch.equals("x86_64")) {
            arch = "amd64";
        }
        String os = System.getProperty("os.name").toLowerCase();
        if (os.contains("windows")) {
            return base + "_windows_" + arch + ".dll";
        } else if (os.contains("mac os x")) {
            return "lib" + base + "_osx_" + arch + ".dylib";
        } else if (os.contains("linux")) {
            return "lib" + base + "_linux_" + arch + ".so";
        } else {
            throw new UnsupportedOperationException("The platform " + os + "/" + arch + " is not supported");
        }
    }

    private static void loadLibrary(String base) {
        String libName = getLibName(base);
        String path = "/native/" + libName;
        System.out.println("Resource path: " + path);
        URL url = PiperService.class.getResource(path);
        if (url == null) {
            throw new UnsupportedOperationException("The platform is not supported.");
        }
        try {
            File tempFile = File.createTempFile(libName, ".lib");
            tempFile.deleteOnExit();
            try (InputStream input = url.openStream()) {
                OutputStream output = new BufferedOutputStream(new FileOutputStream(tempFile));
                int len = 0;
                byte[] buffer = new byte[8192];
                while ((len = input.read(buffer)) > -1)
                    output.write(buffer, 0, len);
                output.close();
                input.close();
                System.load(tempFile.getAbsolutePath());
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public PiperService(String pipelines, String lookups, UdfRepository repo) {
        svcHandle = create(pipelines, lookups, repo.udfMap);
    }

    public void start(String address, short port) {
        start(svcHandle, address, port);
    }

    public void stop() {
        stop(svcHandle);
    }

    @Override
    public void close() throws Exception {
        destroy(svcHandle);
    }


    static Object inc(Object arg) {
        Long n = (Long) arg;
        return n + 42;
    }

    static Object dec(Object arg) {
        Long n = (Long) arg;
        return n - 42;
    }

    public static void main(String[] args) {
        Function1 f = PiperService::inc;
        UdfRepository repo = new UdfRepository()
                .put("inc", (Function1) PiperService::inc)
                .put("dec", (Function1) PiperService::dec);
        try (
                PiperService svc = new PiperService("t(x) | project y=inc(x), z=dec(x);", "", repo)) {
            new Thread(() -> {
                svc.start("localhost", (short) 8000);
            }).start();
            Thread.sleep(5 * 1000);
            svc.stop();
        } catch (
                Exception e) {
            e.printStackTrace();
        }
    }
}
