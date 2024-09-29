// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_car_with_details_query_handler.h"
#include "repository/interface_car_repository.h"
#include "tools/automapper.h"

using namespace Simple;
using namespace Simple::Application::Features::Car::Queries;

GetCarWithDetailsQueryHandler::GetCarWithDetailsQueryHandler(InterfaceCarRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<CarWithDetailsDTO> GetCarWithDetailsQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                const GetCarQuery &query)
{
    Result<CarWithDetailsDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<CarWithDetailsDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetCarQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<CarWithDetailsDTO> GetCarWithDetailsQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                    const GetCarQuery &query)
{
    qDebug() << "GetCarWithDetailsQueryHandler::handleImpl called with id" << query.id;

    // do
    auto carResult = m_repository->getWithDetails(query.id);

    QLN_RETURN_IF_ERROR(CarWithDetailsDTO, carResult)

    Simple::Entities::Car car = carResult.value();

    // map
    auto carWithDetailsDTO = Simple::Tools::AutoMapper::map<Simple::Entities::Car, CarWithDetailsDTO>(car);

    qDebug() << "GetCarWithDetailsQueryHandler::handleImpl done";

    return Result<CarWithDetailsDTO>(carWithDetailsDTO);
}

bool GetCarWithDetailsQueryHandler::s_mappingRegistered = false;

void GetCarWithDetailsQueryHandler::registerMappings()
{
    Simple::Tools::AutoMapper::registerMapping<Simple::Entities::Car, Contracts::DTO::Car::CarWithDetailsDTO>();
}