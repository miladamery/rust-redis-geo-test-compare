package ir.desprime.TestGeoSpatialReactive;

import lombok.RequiredArgsConstructor;
import org.apache.commons.lang3.RandomStringUtils;
import org.springframework.boot.ApplicationArguments;
import org.springframework.boot.ApplicationRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.stereotype.Component;

import java.util.Random;

@SpringBootApplication
public class TestGeoSpatialReactiveApplication {

	public static void main(String[] args) {
		SpringApplication.run(TestGeoSpatialReactiveApplication.class, args);
	}

	@Component
	@RequiredArgsConstructor
	public static class OnStartup implements ApplicationRunner {

		private final GeoService geoService;

		@Override
		public void run(ApplicationArguments args) {

			Random random = new Random();

			// France bounds (approximate values)
			double minLat = 41.303;
			double maxLat = 51.124;
			double minLon = -5.725;
			double maxLon = 9.562;

			System.out.println("Starting to populate data");
			for (int i = 0; i < 500000; i++) {
				double latitude = minLat + (maxLat - minLat) * random.nextDouble();
				double longitude = minLon + (maxLon - minLon) * random.nextDouble();
				geoService.add(
						new Location(
								RandomStringUtils.randomAlphabetic(7),
								latitude,
								longitude
						)
				);
			}
			System.out.println("Date population ended");
		}
	}

}
