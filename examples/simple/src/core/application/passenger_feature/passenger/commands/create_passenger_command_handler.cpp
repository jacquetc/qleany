// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "create_passenger_command_handler.h"
#include "car.h"
#include "passenger/validators/create_passenger_command_validator.h"
#include "qleany/tools/automapper/automapper.h"

using namespace Qleany;
using namespace Simple::Domain;
using namespace Simple::Contracts::DTO::Passenger;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Passenger::Validators;
using namespace Simple::Application::Features::Passenger::Commands;

CreatePassengerCommandHandler::CreatePassengerCommandHandler(InterfacePassengerRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<PassengerDTO> CreatePassengerCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                           const CreatePassengerCommand &request)
{
    Result<PassengerDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<PassengerDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreatePassengerCommand:" << ex.what();
    }
    return result;
}

Result<PassengerDTO> CreatePassengerCommandHandler::restore()
{
    Result<PassengerDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<PassengerDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreatePassengerCommand restore:" << ex.what();
    }
    return result;
}

Result<PassengerDTO> CreatePassengerCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                               const CreatePassengerCommand &request)
{
    qDebug() << "CreatePassengerCommandHandler::handleImpl called";
    Simple::Domain::Passenger passenger;
    CreatePassengerDTO createDTO = request.req;

    if (m_newEntity.isEmpty())
    {
        // Validate the create Passenger command using the validator
        auto validator = CreatePassengerCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(request.req);

        if (Q_UNLIKELY(validatorResult.hasError()))
        {
            return Result<PassengerDTO>(validatorResult.error());
        }

        // Map the create Passenger command to a domain Passenger object and
        // generate a UUID
        passenger =
            Qleany::Tools::AutoMapper::AutoMapper::map<CreatePassengerDTO, Simple::Domain::Passenger>(createDTO);

        // allow for forcing the uuid
        if (passenger.uuid().isNull())
        {
            passenger.setUuid(QUuid::createUuid());
        }

        // Set the creation and update timestamps to the current date and time
        passenger.setCreationDate(QDateTime::currentDateTime());
        passenger.setUpdateDate(QDateTime::currentDateTime());
    }
    else
    {
        passenger = m_newEntity.value();
    }

    // Add the passenger to the repository

    m_repository->beginChanges();
    auto passengerResult = m_repository->add(std::move(passenger));

    if (Q_UNLIKELY(passengerResult.hasError()))
    {
        m_repository->cancelChanges();
        return Result<PassengerDTO>(passengerResult.error());
    }
    const auto &carSchema = Simple::Domain::Car::schema;
    auto rightListResult = m_repository->getEntitiesInRelationOf(carSchema, createDTO.carId(), "passengers");

    if (Q_UNLIKELY(rightListResult.hasError()))
    {
        m_repository->cancelChanges();
        return Result<PassengerDTO>(rightListResult.error());
    }
    auto rightList = rightListResult.value();

    if (createDTO.position() == -1)
    {
        createDTO.setPosition(rightList.size());
    }
    else
    {
        rightList.insert(createDTO.position(), passengerResult.value());
    }
    auto newListResult =
        m_repository->updateEntitiesInRelationOf(carSchema, createDTO.carId(), "passengers", rightList);

    if (Q_UNLIKELY(newListResult.hasError()))
    {
        m_repository->cancelChanges();
        return Result<PassengerDTO>(newListResult.error());
    }

    m_repository->saveChanges();

    m_newEntity = passengerResult;

    auto passengerDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Passenger, PassengerDTO>(passengerResult.value());
    emit passengerCreated(passengerDTO);

    qDebug() << "Passenger added:" << passengerDTO.id();

    // Return the DTO of the newly created Passenger as a Result object
    return Result<PassengerDTO>(passengerDTO);
}

Result<PassengerDTO> CreatePassengerCommandHandler::restoreImpl()
{

    auto deleteResult = m_repository->remove(m_newEntity.value().id());

    if (Q_UNLIKELY(deleteResult.hasError()))
    {
        qDebug() << "Error deleting Passenger from repository:" << deleteResult.error().message();
        return Result<PassengerDTO>(deleteResult.error());
    }

    emit passengerRemoved(deleteResult.value());

    qDebug() << "Passenger removed:" << deleteResult.value();

    return Result<PassengerDTO>(PassengerDTO());
}

bool CreatePassengerCommandHandler::s_mappingRegistered = false;

void CreatePassengerCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Passenger,
                                                           Contracts::DTO::Passenger::PassengerDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Passenger::CreatePassengerDTO,
                                                           Simple::Domain::Passenger>();
}
