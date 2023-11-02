// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_car_query_handler.h"
#include "qleany/tools/automapper/automapper.h"
#include "repository/interface_car_repository.h"

using namespace Qleany;
using namespace Simple::Application::Features::Car::Queries;

GetCarQueryHandler::GetCarQueryHandler(InterfaceCarRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<CarDTO> GetCarQueryHandler::handle(QPromise<Result<void>> &progressPromise, const GetCarQuery &query)
{
    Result<CarDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<CarDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetCarQuery:" << ex.what();
    }
    return result;
}

Result<CarDTO> GetCarQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise, const GetCarQuery &query)
{
    qDebug() << "GetCarQueryHandler::handleImpl called with id" << query.id;

    // do
    auto carResult = m_repository->get(query.id);

    QLN_RETURN_IF_ERROR(CarDTO, carResult)

    // map
    auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Car, CarDTO>(carResult.value());

    qDebug() << "GetCarQueryHandler::handleImpl done";

    return Result<CarDTO>(dto);
}

bool GetCarQueryHandler::s_mappingRegistered = false;

void GetCarQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Car, Contracts::DTO::Car::CarDTO>(true,
                                                                                                             true);
}