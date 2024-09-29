// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "persistence_registration.h"
#include "database/database_context.h"
#include "database/database_table_group.h"

#include "repository/brand_repository.h"
#include "repository/car_repository.h"
#include "repository/client_repository.h"
#include "repository/generic_repository.h"
#include "repository/passenger_repository.h"
#include "repository/repository_provider.h"

using namespace FrontEnds using namespace FrontEnds::Persistence::Database;
using namespace FrontEnds::Persistence;
using namespace FrontEnds::Persistence::Repository;

PersistenceRegistration::PersistenceRegistration(QObject *parent)
    : QObject{parent}
{
    QSharedPointer<DatabaseContext> context(new DatabaseContext());

    // database tables:

    auto *brandDatabaseTableGroup = new DatabaseTableGroup<FrontEnds::Entities::Brand>(context);
    auto *carDatabaseTableGroup = new DatabaseTableGroup<FrontEnds::Entities::Car>(context);
    auto *clientDatabaseTableGroup = new DatabaseTableGroup<FrontEnds::Entities::Client>(context);
    auto *passengerDatabaseTableGroup = new DatabaseTableGroup<FrontEnds::Entities::Passenger>(context);

    Result<void> initResult = context->init();

    if (initResult.hasError()) {
        Error error = initResult.error();
        qCritical() << error.className() + "\n"_L1 + error.code() + "\n"_L1 + error.message() + "\n"_L1 + error.data();
    }

    // repositories:

    BrandRepository *brandRepository = new BrandRepository(brandDatabaseTableGroup);
    CarRepository *carRepository = new CarRepository(carDatabaseTableGroup, brandRepository, passengerRepository);
    ClientRepository *clientRepository = new ClientRepository(clientDatabaseTableGroup, passengerRepository);
    PassengerRepository *passengerRepository = new PassengerRepository(passengerDatabaseTableGroup);

    // register repositories:

    RepositoryProvider::instance()->registerRepository("brand", brandRepository);
    RepositoryProvider::instance()->registerRepository("car", carRepository);
    RepositoryProvider::instance()->registerRepository("client", clientRepository);
    RepositoryProvider::instance()->registerRepository("passenger", passengerRepository);
}

RepositoryProvider *PersistenceRegistration::repositoryProvider()
{
    return RepositoryProvider::instance();
}